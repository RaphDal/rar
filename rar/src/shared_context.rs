use std::{
    collections::VecDeque,
    future::Future,
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc, RwLock,
    },
    thread,
};

use crate::task::Task;

#[derive(Clone, Debug)]
pub(crate) struct SharedContext(pub(crate) Arc<SharedContextInner>);

impl SharedContext {
    pub(crate) fn new() -> Self {
        Self(Arc::new(SharedContextInner::new()))
    }

    pub(crate) fn spawn<F: Future<Output = ()> + Send + 'static>(&self, future: F) {
        if !self.0.active.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }

        self.0
            .working_tasks
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.0.add_task(Arc::new(Task::new(future, self.clone())));
    }

    pub(crate) fn reset(&self) {
        self.0
            .active
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

#[derive(Debug)]
pub(crate) struct SharedContextInner {
    pub(crate) pending_tasks: RwLock<VecDeque<Arc<Task>>>,
    pub(crate) threads: RwLock<VecDeque<Box<thread::Thread>>>,
    pub(crate) scheduler_thread: thread::Thread,
    pub(crate) active: AtomicBool,
    pub(crate) working_tasks: AtomicU64,
}

impl SharedContextInner {
    pub fn new() -> Self {
        Self {
            pending_tasks: RwLock::new(VecDeque::new()),
            threads: RwLock::new(VecDeque::new()),
            scheduler_thread: thread::current(),
            active: AtomicBool::new(true),
            working_tasks: AtomicU64::new(0),
        }
    }

    pub(crate) fn add_task(&self, task: Arc<Task>) {
        {
            self.pending_tasks.write().unwrap().push_back(task);
        }
        let mut available_threads = self.threads.write().unwrap();
        if !available_threads.is_empty() {
            let thread = available_threads.pop_front().unwrap();
            thread.unpark();
        }
    }
}
