 rust
macro_rules! foo {
    ( $( $some:tt )* ) => {
        macro_rules! bar {
            ( $( $any:tt )* ) => { $( $some $any )* };
        }
    };
}
