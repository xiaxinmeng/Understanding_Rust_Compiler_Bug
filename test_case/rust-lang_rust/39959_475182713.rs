rust
trait Trait {}
impl<'a, T> Trait for &'a Vec<T> where &'a T: Trait {}

fn call<E: Trait>() {}
    
fn foo() {
    call::<&Vec<_>>();
}
