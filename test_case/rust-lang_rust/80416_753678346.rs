rust
pub struct CustomSkip<I> {
    iter: I,
    n: Option<usize>,
}

impl<I> CustomSkip<I> {
    pub fn new(iter: I, n: usize) -> CustomSkip<I> {
        CustomSkip { iter, n: Some(n) }
    }
}

impl<I: Iterator> Iterator for CustomSkip<I> {

    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if let Some(n) = self.n.take() {
            self.iter.nth(n)
        } else {
            self.iter.next()
        }
    }
}

trait SkipExt: Iterator + Sized {
    fn custom_skip(self, n: usize) -> CustomSkip<Self> {
        CustomSkip::new(self, n)
    }
}

impl<I: Iterator> SkipExt for I {}
