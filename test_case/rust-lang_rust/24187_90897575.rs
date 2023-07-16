 rust
impl Broken {
    fn decompose(&mut self) -> (&mut usize, &DoesNotCopy) {
        (&mut self.x, &self.y)
    }

    fn do_stuff(&mut self) -> (usize, &DoesNotCopy) {
        let (ref mut x, ref y) = self.decompose();
        (std::mem::replace(x, 0), y)
    }
}
