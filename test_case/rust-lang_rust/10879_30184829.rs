
trait NoRc<T>: DoesNotContain<ReferenceCounted<T>> { }
impl<X: DoesNotContain<ReferenceCounted<T>> NoRc<T> for X { }
