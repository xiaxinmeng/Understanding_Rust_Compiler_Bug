
    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if unlikely(self.n > 0) {
            self.iter.nth(self.n-1);
            self.n=0;
        }
        self.iter.next()
    }
