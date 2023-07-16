rust
const fn foo() -> &'static str {
    "Hello, World!"
}

const FOO: &'static str = foo();

fn test() -> &'static [&'static str] {
    &[
        "Line 1",
        FOO,
    ]
}
