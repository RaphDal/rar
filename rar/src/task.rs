use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{RawWaker, Waker},
};

use crate::{shared_context::SharedContext, waker::VTABLE};

pub(crate) struct Task {
    pub future: Arc<Mutex<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>,
    pub shared_context: SharedContext,
}

impl Task {
    pub fn new<F: Future<Output = ()> + Send + 'static>(
        future: F,
        shared_context: SharedContext,
    ) -> Self {
        Self {
            future: Arc::new(Mutex::new(Box::pin(future))),
            shared_context,
        }
    }

    pub fn waker(self: Arc<Self>) -> Waker {
        let ptr = Arc::into_raw(self) as *const ();
        unsafe { Waker::from_raw(RawWaker::new(ptr, VTABLE)) }
    }
}

impl Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Task").finish()
    }
}
