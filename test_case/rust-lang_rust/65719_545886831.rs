rust
let mut curr: *const Waiter = ...;
while !curr.is_null() {
    let thread = unsafe {
        let next = (*curr).next;
        let thread = (*curr).thread.take().unwrap();
        (*curr).signaled.store(true, Release);
        thread
    }
    thread.unpark();
}
