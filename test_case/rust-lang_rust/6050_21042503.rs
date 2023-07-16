 rust
pub struct Counter<A> {
    state: A,
    step: A
}

impl<A> Counter<A> {
    #[inline(always)]
    fn new(start: A, step: A) -> Counter<A> {
        Counter{state: start, step: step}
    }
}

impl<A: Add<A, A> + Clone> Iterator<A> for Counter<A> {
    #[inline(always)]
    fn next(&mut self) -> Option<A> {
        let result = self.state.clone();
        self.state = self.state + self.step;
        Some(result)
    }
}

fn main() { }
