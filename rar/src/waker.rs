use std::{
    mem,
    sync::Arc,
    task::{RawWaker, RawWakerVTable},
};

use crate::task::Task;

pub(crate) const VTABLE: &'static RawWakerVTable =
    &RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop);

unsafe fn incr_ref_count(ptr: *const ()) {
    let arc = mem::ManuallyDrop::new(unsafe { Arc::from_raw(ptr as *const Task) });
    let _arc_clone: mem::ManuallyDrop<_> = arc.clone();
}

unsafe fn waker_clone(ptr: *const ()) -> RawWaker {
    incr_ref_count(ptr);
    RawWaker::new(ptr, VTABLE)
}

unsafe fn waker_wake(ptr: *const ()) {
    let task: Arc<Task> = Arc::from_raw(ptr.cast());
    let shared_context = task.shared_context.clone();
    shared_context.0.add_task(task);
}

unsafe fn waker_wake_by_ref(ptr: *const ()) {
    let task: Arc<Task> = Arc::from_raw(ptr.cast());
    task.shared_context.0.add_task(task.clone());
}

unsafe fn waker_drop(ptr: *const ()) {
    let arc = Arc::from_raw(ptr.cast::<Task>());
    drop(arc);
}
