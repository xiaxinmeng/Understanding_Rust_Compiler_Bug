rust
struct Foo<'a, const N: usize = {
    let x: &'a ();
    3
}>(&'a ());
