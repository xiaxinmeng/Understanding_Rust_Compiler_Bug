rust
// format_args!("hello {}", world)
match match (&world,) {
    (arg0,) => [::core::fmt::ArgumentV1::new(
        arg0,
        ::core::fmt::Display::fmt,
    )],
} {
    ref args => unsafe { ::core::fmt::Arguments::new_v1(&["hello "], args) },
};
