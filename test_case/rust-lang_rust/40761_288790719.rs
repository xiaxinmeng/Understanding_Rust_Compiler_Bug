rust
trait Trait {}

trait Associate {
    type Assoc;
}

impl<T> Trait for T {}

impl Trait for Associate<Assoc=()> {}   // ok
impl<T> Trait for Associate<Assoc=T> {} // error
