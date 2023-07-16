rust
type Foo = impl std::fmt::Debug;

fn foo(b: bool) -> Foo {
    let x: Foo = if b {
        vec![42_i32]
    } else {
        std::iter::empty().collect()
        //~^ ERROR `Foo` cannot be built from an iterator over elements of type `_`
    };
    x
}
