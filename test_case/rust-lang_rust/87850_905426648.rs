rust
use Send as SomeAutoTrait;
macro_rules! SomeExistential {() => ( impl Sized )}

fn foo() -> SomeExistential!() {
    // check if `typeof![foo()] : SomeAutoTrait`
    let _: &dyn SomeAutoTrait = &foo();
}
