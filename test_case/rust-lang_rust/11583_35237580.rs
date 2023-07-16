
#[unsafe_destructor]
impl<T> Drop for AtomicOption<T> {
    fn drop(&mut self) {
        let _ = self.take(SeqCst);
    }
}
