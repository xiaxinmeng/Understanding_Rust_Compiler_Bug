rust
macro_rules! normal_macro {
    ($($input:tt)*) => {
        let _ = move | a : String | $($input)*;
    } ;
}
