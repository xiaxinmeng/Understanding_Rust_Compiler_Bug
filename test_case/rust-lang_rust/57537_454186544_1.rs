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
    s.write_fmt($crate::fmt::Arguments::new_v1_formatted(
        &[""],
        &match (&"foo",) {
            (arg0,) => [$crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt)],
        },
        &[$crate::fmt::rt::v1::Argument {
            position: $crate::fmt::rt::v1::Position::At(0usize),
            format: $crate::fmt::rt::v1::FormatSpec {
                fill: ' ',
                align: $crate::fmt::rt::v1::Alignment::Unknown,
                flags: 0u32,
                precision: $crate::fmt::rt::v1::Count::Implied,
                width: $crate::fmt::rt::v1::Count::Implied,
            },
        }],
    ));
}
