rust
macro_rules! ignore {
    ($($tt:tt)*) => ();
}

macro_rules! count {
    ( $( $i:stmt ),* ) => {{
        0 $( + { ignore!($i); 1 } )*
    }};
}
