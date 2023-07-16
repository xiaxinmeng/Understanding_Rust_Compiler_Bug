
pub struct DerefError(&dyn Deref<Target = dyn Error+Send+Sync+'static>+DerefMut+Drop)
impl Drop for DerefError {
    fn drop(&mut self) { self.drop() }
}
impl Error for DerefError {
    // delegate all methods to self.deref()
}
