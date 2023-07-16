 rust
macro_rules! foo_macro {
    ($(#[$attr:meta])* struct $name:ident) => {
        $(#[$attr])*
        pub struct $name;
    }
}

foo_macro!(
    /// Doc comment for Foo struct.
    struct Foo
);

foo_macro!(
    /// Doc comment for Bar struct.
    struct Bar
);

fn main() {
}
