
struct Counter;

impl FnMut(&int) -> bool for Counter {
    extern "rust-call" fn call_mut(&mut self, (&x,): (&int,)) -> bool {}
}
