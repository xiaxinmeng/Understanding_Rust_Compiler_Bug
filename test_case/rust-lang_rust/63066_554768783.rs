rust
impl Trait for impl AnotherTrait { ... }
impl<T: AnotherTrait> Trait for T { ... }
//or
impl Trait for dyn AnotherTrait { ... }
impl<T: AnotherTrait> Trait for T { ... }
