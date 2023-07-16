rust
// Table tests for a function called "baz" on a result type that is being checked
// with a match
macro_rules! test_invalid_baz {
    ( $( $num:ident: [$arg:expr, $err:expr] ),+ ) => {
        $(
            #[test]
            fn $num() {
                //  with $arg as the input, expect Err($err) as the output
                match baz($arg) {
                    Err($err) => {},
                    _ => panic!("Errors did not match"),
                }
            }
        )*
    };
}

test_invalid_split!([test0, "foo", Error::Bar])
