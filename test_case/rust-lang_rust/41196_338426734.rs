rust
if self.counter <= 10 {
    cpu_relax(4 << self.counter);
} else {
    thread_yield();
}
