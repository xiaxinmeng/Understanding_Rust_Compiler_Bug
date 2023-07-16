rust
/// Fails to build with a weird message about type size issues
fn identity<T: Sized>(x: T) -> T { unsafe { core::mem::transmute(x) } }
