rust
trait TraitA { type Item: ?Sized; }

trait TraitB<T> { }

impl<X: TraitA> TraitB<X> for X::Item { }

impl TraitA for () {
    type Item = dyn TraitB<()>;
}
