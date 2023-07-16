rust
match some_boxed_enum {
    box Foo::Bar { ref mut a, ref b } => { ... }
}
