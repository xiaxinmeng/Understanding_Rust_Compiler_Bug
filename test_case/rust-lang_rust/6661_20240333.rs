
#[fixedstacksegment]
fn call_native_fn() {
    libc::free();
    // Wasting a huge amount of stack by yielding to the scheduler
    yield();
}
