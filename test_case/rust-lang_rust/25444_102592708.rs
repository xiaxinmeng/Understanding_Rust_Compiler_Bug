 rust
macro_rules! foo {
    ( $($a:tt)* $($b:tt)* ) => { }; //~ ERROR sequence repetition followed by another sequence repetition
    ( $($a:item)* $($b:item)* ) => { }; //~ ERROR sequence repetition followed by another sequence repetition
}
