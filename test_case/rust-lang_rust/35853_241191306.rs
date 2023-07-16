 rust
macro_rules! foo {
    ( $( $some:tt )* ) => {
        macro_rules! bar {
            ( $( $some )* $( foo )* $( $any:tt bar )* ) => { $( $any )* };
        }
    };
}
