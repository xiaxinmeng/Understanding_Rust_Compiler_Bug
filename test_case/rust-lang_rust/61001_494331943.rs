rust
macro_rules! foo {
    ($(#[$m:meta])* $i:ident) => {
        $(#[$m])*
        pub struct $i;
    }
}

foo!(
    /// Stuff
    /// More stuff
    Alpha
);
