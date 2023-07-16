 rust
impl CoerceUnsized<Pointer<Trait>> for Pointer<Trait> {}
impl<T: Trait> CoerceUnsized<Pointer<Trait>> for Pointer<T> {}
