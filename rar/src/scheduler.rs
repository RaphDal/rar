use std::{future::Future, thread};

use crate::shared_context::SharedContext;

pub struct Scheduler {
    shared_context: SharedContext,
}

impl Scheduler {
    pub(crate) fn new(shared_context: SharedContext) -> Self {
        Self { shared_context }
    }

    pub fn block_on<F: Future<Output = ()> + Send + 'static>(&self, future: F) {
        self.shared_context.reset();
        self.shared_context.spawn(future);
        thread::park();
    }
}
