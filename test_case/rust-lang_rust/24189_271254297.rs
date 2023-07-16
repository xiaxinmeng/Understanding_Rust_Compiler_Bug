rust
macro_rules! foo {
    ( $( > )* $x:ident ) => { };
}
foo!( > a );
