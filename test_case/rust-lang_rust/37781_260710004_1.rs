 rust
    ($name:ident, $submac:ident!( $($args:tt)* )) => (
        fn $name( i: &[u8] ) -> $crate::IResult<&[u8], &[u8], u32> {
            $submac!(i, $($args)*)
        }
    );
