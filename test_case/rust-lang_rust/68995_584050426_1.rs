rust
match self.inner.next() {
    None => {
        assert_eq!(self.size, 0);
        None
    }
    Some(x) => {
        self.size = self.size.checked_sub(1).unwrap();
        Some(x)
    }
}
