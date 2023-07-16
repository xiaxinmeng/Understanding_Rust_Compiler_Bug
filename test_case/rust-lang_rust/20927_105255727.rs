 rust
struct Foo<T> { ... }

/* manual impl */
impl<T> PartialEq for Foo<T> { ... }

#[derive(PartialEq)]
struct Bar<T> {
    x: Foo<T>,
}
