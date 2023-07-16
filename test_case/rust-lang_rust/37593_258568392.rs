 rust
 // Note: Implementations on generic type parameters are called "blanket impls"
impl<T> MyTrait for T where T: Trait1 + Trait2 ... { /* ... */ }
