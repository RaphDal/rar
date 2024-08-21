use std::{
    future::Future,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread,
    time::Duration,
    vec,
};

#[derive(Debug)]
enum TimerStatus {
    Pending,
    Started,
    Complete,
}

#[derive(Debug)]
struct SharedState {
    status: TimerStatus,
    wakers: Vec<Waker>,
}

#[derive(Debug)]
pub struct TimerFuture {
    duration: Duration,
    state: Arc<Mutex<SharedState>>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            state: Arc::new(Mutex::new(SharedState {
                status: TimerStatus::Pending,
                wakers: vec![],
            })),
        }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut shared_state = self.state.lock().unwrap();
        match shared_state.status {
            TimerStatus::Complete => Poll::Ready(()),
            TimerStatus::Pending => {
                shared_state.status = TimerStatus::Started;
                shared_state.wakers.push(cx.waker().clone());
                let duration = self.duration.clone();
                let shared_state_ref = self.state.clone();
                thread::spawn(move || {
                    thread::sleep(duration);
                    let mut shared_state = shared_state_ref.lock().unwrap();
                    shared_state.status = TimerStatus::Complete;
                    for waker in shared_state.wakers.iter() {
                        waker.wake_by_ref();
                    }
                });
                Poll::Pending
            }
            TimerStatus::Started => {
                shared_state.wakers.push(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}
