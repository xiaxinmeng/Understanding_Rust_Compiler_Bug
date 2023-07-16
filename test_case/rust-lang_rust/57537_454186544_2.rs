rust
fn foo(s: &mut String) {
    // parsed as `ArgumentIs(0)`
    s.write_fmt($crate::fmt::Arguments::new_v1(
        &[""],
        &match (&"foo",) {
            (arg0,) => [$crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt)],
        },
    ));
    // parsed as `ArgumentImplicitlyIs(0)`
    s.write_fmt($crate::fmt::Arguments::new_v1(
        &[""],
        &match (&"foo",) {
            (arg0,) => [$crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt)],
        },
    ));
}
