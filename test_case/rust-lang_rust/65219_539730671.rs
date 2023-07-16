rust
pub struct SizeHint<T> {
    hint: (usize, Option<usize>),
    item: PhantomData<T>,
}

impl<T> Iterator for SizeHint<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.hint
    }
}
