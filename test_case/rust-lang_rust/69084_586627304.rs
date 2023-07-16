rust
macro_rules! foo {
    ($( $tokens:tt )*) => { $( $tokens )* }
}

/// Docs for struct S
foo! {
    pub struct S;
}
