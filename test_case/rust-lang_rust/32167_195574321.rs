 rust
struct Bar;
mod foo {
    use Bar;
    use foo::Bar as Baz; // ... we will want this to resolve (a private import),
    use foo::Ok; // but not this (a name from the prelude).
}
