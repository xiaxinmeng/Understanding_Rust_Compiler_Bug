rust
trait Functor<A> {
    type Associated<B>: Iterator<Item = B>;
}
