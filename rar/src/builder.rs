use crate::{context::start_context, scheduler::Scheduler, shared_context::SharedContext};

pub struct Builder {
    pub threads: usize,
}

impl Builder {
    pub fn new() -> Self {
        Self { threads: 1 }
    }

    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    pub fn build(self) -> Scheduler {
        let shared_context = SharedContext::new();
        for id in 0..self.threads {
            start_context(id as u64, &shared_context);
        }
        Scheduler::new(shared_context)
    }
}
