 rust
macro_rules! test_cons {
    (
        $ty:ident {
            array: $arr:tt,
            $(
                $from:ident: $val:expr,
            )*
        }
    ) => {
        $(
            test_arrays! { ($ty::$from)($val) == $arr }
        )*
    }
}
