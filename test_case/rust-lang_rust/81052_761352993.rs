
impl<R: Read> Iterator for Bytes<BufReader<R>> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner.buffer().len(), None)
    }
}
