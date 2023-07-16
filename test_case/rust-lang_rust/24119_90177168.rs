 rust
fn next(&mut self) -> Option<A> {
    self.start.step(&A::one()).map(|mut n| {
        mem::swap(&mut n, &mut self.start);
        n
    })
}
