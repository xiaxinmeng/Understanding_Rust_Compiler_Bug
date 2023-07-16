rust
impl<'a, K, V> Iterator for Drain<'a, K, V> {
    // ...
    #[inline]
    fn next(&mut self) -> Option<(SafeHash, K, V)> {
        self.iter.next().map(|raw| {
            unsafe {
                self.table.as_mut().size -= 1;
                let (k, v) = ptr::read(raw.pair());
                (SafeHash { hash: ptr::replace(&mut *raw.hash(), EMPTY_BUCKET) }, k, v)
            }
        })
    }
    // ...
}
