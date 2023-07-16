rust
pub fn once_with<A, F: FnOnce() -> A>(gen: F) -> OnceWith<F> {…}
pub struct OnceWith<F> {…}
impl<A, F: FnOnce() -> A> Iterator for OnceWith<F> {…}
impl<A, F: FnOnce() -> A> DoubleEndedIterator for OnceWith<F> {…}
impl<A, F: FnOnce() -> A> ExactSizeIterator for OnceWith<F> {…}
impl<A, F: FnOnce() -> A> FusedIterator for OnceWith<F> {…}
unsafe impl<A, F: FnOnce() -> A> TrustedLen for OnceWith<F> {…}
