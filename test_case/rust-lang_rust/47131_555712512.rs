rust
trait Trait<S> {}

type Alias = ();

trait Trait2 {
    type Type: Trait<Alias>;
}
