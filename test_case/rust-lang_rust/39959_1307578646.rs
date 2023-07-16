Rust
struct Struct<T>(T);

trait Trait {}
impl<T> Trait for Struct<T> where T: Trait {}
impl Trait for u8 {}

fn call<E: Trait>() {}
    
fn foo() {
    call::<Struct<_>>();
}
