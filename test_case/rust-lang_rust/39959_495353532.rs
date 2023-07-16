rust
struct Struct<T> where T: Sized {
  t: T
}

trait Trait {}
impl<'a, T> Trait for &'a Struct<T> where &'a T: Trait {}

fn call<E: Trait>() {}
    
fn foo() {
    call::<&Struct<_>>();
}
