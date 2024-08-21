use std::{
    cell::RefCell,
    future::Future,
    sync::{atomic::Ordering, Arc},
    thread,
};

use crate::{shared_context::SharedContext, task::Task};

#[derive(Debug)]
pub struct Context {
    _id: u64,
}

thread_local! {
    pub(crate) static SHARED_CONTEXT: RefCell<Option<SharedContext>> =  RefCell::new(None);
}

pub fn spawn<F: Future<Output = ()> + Send + 'static>(future: F) {
    SHARED_CONTEXT.with(|context| {
        let c = context.clone();
        if let Some(shared_context) = c.take() {
            shared_context.spawn(future);
        }
    });
}

impl Context {
    pub(crate) fn start(&self, shared_context: SharedContext) {
        loop {
            let task: Option<Arc<Task>> = {
                let task = shared_context.0.pending_tasks.write().unwrap().pop_front();
                if matches!(task, None) {
                    {
                        let mut available_threads = shared_context.0.threads.write().unwrap();
                        available_threads.push_back(Box::new(thread::current()));
                    }
                }
                task
            };

            match task {
                Some(task) => {
                    self.execute_task(&shared_context, &task);
                }
                None => {
                    if shared_context.0.working_tasks.load(Ordering::Relaxed) == 0 {
                        shared_context.0.scheduler_thread.unpark();
                    }

                    thread::park();
                }
            }
        }
    }

    fn execute_task(&self, shared_context: &SharedContext, task: &Arc<Task>) {
        let mut future = task.future.lock().unwrap();

        let waker = Arc::clone(&task).waker();
        let mut context = core::task::Context::from_waker(&waker);

        let state = future.as_mut().poll(&mut context);
        if state.is_ready() {
            let count = shared_context
                .0
                .working_tasks
                .fetch_sub(1, Ordering::Relaxed)
                - 1;
            if count == 0 {
                shared_context.0.active.store(false, Ordering::SeqCst);
            }
        }
    }
}

pub(crate) fn start_context(id: u64, shared_context: &SharedContext) {
    let shared_context = shared_context.clone();
    thread::spawn(move || {
        SHARED_CONTEXT.with(|context| {
            context.replace(Some(shared_context.clone()));
        });
        let context = Context { _id: id };
        context.start(shared_context);
    });
}
