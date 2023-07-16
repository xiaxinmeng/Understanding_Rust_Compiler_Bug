rust
trait Trait {}

macro_rules! impl_trait_for {
    ($name:path) => { impl<T> Trait for $name {} }
}

impl_trait_for!(Result<T, T>);
