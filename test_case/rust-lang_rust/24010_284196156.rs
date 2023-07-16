 rust
trait A {
    type Assoc;
    // Pretend there are several more associated types here to make uses of `Box<A>` inconvenient.
}

trait B: A<Assoc=()> {}

fn takes_b(b: Box<B>) {}
