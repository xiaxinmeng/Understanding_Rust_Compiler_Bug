rust
trait Trait {}

fn share<T: Send>(t: &T) {}

fn foo() -> impl Trait {
    let a = foo();
    share(&a);
}
