rust
let Foo {
    field_a,
    field_b,
    field_c,
    #[cfg(test)]
    __test_exhaustive: _,
    #[cfg(not(test))]
    ..
} = foo;
