 Rust
trait Pointer { type Pointee; }
impl<T:?Sized> Pointer for *const T { type Pointee = T; }
type __CFArray = <CFArrayRef as Pointer>::Pointee;
