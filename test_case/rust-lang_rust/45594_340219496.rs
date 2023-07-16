rust
    fn fold1<F>(mut self, f: F) -> Option<Self::Item>
        where F: FnMut(Self::Item, Self::Item) -> Self::Item,
              Self: Sized,
    {
        self.next().map(move |x| self.fold(x, f))
    }
