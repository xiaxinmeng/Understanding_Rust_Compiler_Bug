rust
impl Bencher {
    /// Callback for benchmark functions to run in their body.
    pub fn iter<I, O, F>(&mut self, input: I, mut inner: F)
    where
        I: Copy,
        F: FnMut(I) -> O,
    {
        // Omitting the measurements, statistics, etc.
        loop {
            black_box(f(black_box(input)))
        }
    }
