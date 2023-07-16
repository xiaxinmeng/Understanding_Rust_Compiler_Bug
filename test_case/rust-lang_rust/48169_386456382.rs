rust
pub fn repeat_with<A, F: FnMut() -> A>(repeater: F) -> RepeatWith<F> { /*…*/ }
pub struct RepeatWith<F> { /*…*/ }
impl<A, F: FnMut() -> A> Iterator for RepeatWith<F> { type Item = A; /* … */ }
impl<A, F: FnMut() -> A> DoubleEndedIterator for RepeatWith<F> { /*…*/ }
impl<A, F: FnMut() -> A> FusedIterator for RepeatWith<F> {}
unsafe impl<A, F: FnMut() -> A> TrustedLen for RepeatWith<F> {}
