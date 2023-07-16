rust
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.a.next() {
            return Some(item);
        }
        if let Some(b) = self.b.take() {
            self.a = b;
            self.a.next()
        } else {
            None
        }
    }
