rust
match (&mut *some_boxed_enum,) {
    (&mut Foo::Bar { ref mut a, ref b },) => { ... }
}
