rust
fn write(&self) {
    if self.guard.compare_and_swap(false, true, Ordering::Acquire) == false {
        ptr:write_volatile(...)
        self.guard.store(false, Ordering::Release);
    }
}

fn read(&self) {
    if self.guard.compare_and_swap(false, true, Ordering::Acquire) == false {
        ptr:read_volatile(...)
        self.guard.store(false, Ordering::Release);
    }
}
