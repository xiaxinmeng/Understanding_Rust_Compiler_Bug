
916 | |                 #[doc = concat!("assert_eq!(", stringify!($Ty), "::MIN, ", stringify!($Int), "::MIN;")]
    | |_______________________^
...
...
924 | / nonzero_constants_signed! {
925 | |     NonZeroI8(i8);
926 | |     NonZeroI16(i16);
927 | |     NonZeroI32(i32);
930 | |     NonZeroIsize(isize);
931 | | }
    | |_- in this macro invocation
    |
    |
    = note: `-D rustdoc::invalid-rust-codeblocks` implied by `-D warnings`
    = help: mark blocks that do not contain Rust code as text: 