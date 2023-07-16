rust
macro_rules! breakme {
    ( $name:ident ) => { breakme!($name, stringify!($name), $); };
    ( $name:ident, $name_str:expr, $d:tt ) => {
        #[doc = $name_str]
        #[macro_export]
        macro_rules! $name {
            ( $format:expr, $d( $arg:tt ),+ ) => {
                format!($format, $d( $arg ),+);
            }
        }
    }
}

breakme!(hello);
