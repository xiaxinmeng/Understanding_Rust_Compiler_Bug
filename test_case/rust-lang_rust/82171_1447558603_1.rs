rust
pub type ReturnTy<T> = impl Iterator<Item = String> + 'static;
pub fn foo<T>(a: T) -> ReturnTy<T>
// ...
