rust
fn main() {
    let test = &"hello".to_owned();
    let args = ::std::fmt::Arguments::new_v1_formatted(
        &[""],
        &match (&test,) {
            (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
        },
        &[::std::fmt::rt::v1::Argument {
            position: ::std::fmt::rt::v1::Position::At(0usize),
            format: ::std::fmt::rt::v1::FormatSpec {
                fill: ' ',
                align: ::std::fmt::rt::v1::Alignment::Unknown,
                flags: 0u32,
                precision: ::std::fmt::rt::v1::Count::Implied,
                width: ::std::fmt::rt::v1::Count::Implied,
            },
        }],
    );
}
