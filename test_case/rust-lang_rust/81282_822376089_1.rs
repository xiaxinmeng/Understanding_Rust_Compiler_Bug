rust
let Foo {
    field_a,
    field_b,
    field_c,
    // could also be warn(reachable) + `-D warnings` in CI
    #[cfg_attr(test, deny(reachable))]
    ..
} = foo;
