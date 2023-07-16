
error: cannot specialize on trait `io::Read`
     |
     |
2480 | / impl<R: Read> Iterator for Bytes<BufReader<R>> {
2481 | |     fn size_hint(&self) -> (usize, Option<usize>) {
2482 | |         (self.inner.buffer().len(), None)
2484 | | }
     | |_^

error: aborting due to previous error
error: aborting due to previous error

error: could not compile `std`
