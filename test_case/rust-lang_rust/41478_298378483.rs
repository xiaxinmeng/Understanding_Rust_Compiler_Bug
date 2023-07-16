rust
struct V<S>(S);
trait An {
    type U;
}
trait F<A> {
}
impl<A: An> F<A> for V<<A as An>::U> {
}
