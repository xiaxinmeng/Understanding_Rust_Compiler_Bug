 rust
struct Foo;

mod sub {
    use super::*;

    type Bar = Foo; // Error: use of undeclared type name Foo
}
