rust
const fn foo() -> &'static str {
    "Hello, World!"
}

fn test() -> &'static [&'static str] {
    &[
        "Line 1",
        foo(),
    ]
}
