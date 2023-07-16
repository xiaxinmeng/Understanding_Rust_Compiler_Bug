rust
// sealed for good measure
mod private {
    pub auto trait NotSame {}
    impl<T> !NotSame for (T, T) {}
}
use NotSame;

trait ReqDifferent<T> where (T, Self): NotSame {}
trait ReqSame<T> where (T, Self): !NotSame {}
