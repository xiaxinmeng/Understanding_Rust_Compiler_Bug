rust
    fn for_each<F, R, R2>(self, f: F) -> R where
        Self: Sized,
        F: FnMut(Self::Item) -> R2, 
        R: FromIterator<R2>,
    {
        self.map(f).collect()
    }
