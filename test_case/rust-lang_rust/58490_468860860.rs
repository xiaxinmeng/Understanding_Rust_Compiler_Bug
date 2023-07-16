rust
#![allow(duplicate_macro_exports)]
macro_rules! a {
    ( @1 $i:item ) => {
        a! { @2 $i }
    };
    ( @2 $i:item ) => {
        $i
    };
}
mod b {
    a! {
        @1
        #[macro_export]
        macro_rules! b { () => () }
    }
    #[macro_export]
    macro_rules! b { () => () }
}
mod c {
    use crate::b;
}
