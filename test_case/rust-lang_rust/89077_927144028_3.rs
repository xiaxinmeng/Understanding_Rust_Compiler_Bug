text
    = note: error from rustc: this file contains an unclosed delimiter
    = note: this error originates in the macro `nonzero_constants_signed` (in Nightly builds, run with -Z macro-backtrace for more info)
error: could not parse code block as Rust code
   --> library/core/src/num/nonzero.rs:948:17
    |
    |
948 | /                 #[doc = concat!("The minimum value for a `", stringify!($Ty), "`.")]
949 | |                 /// Note: While most integer types are defined for every whole number between MIN and
950 | |                 /// MAX, signed non-zero integers are a special case. They have a 'gap' at 0.
951 | |                 /// # Examples
...   |
954 | |                 #[doc = concat!("assert_eq!(", stringify!($Ty), "::MIN, ", stringify!($Int), "::MIN;")]
    | |_______________________^
...
...
962 | / nonzero_constants_unsigned! {
963 | |    NonZeroU8(u8);
964 | |     NonZeroU16(u16);
965 | |     NonZeroU32(u32);
968 | |     NonZeroUsize(usize);
969 | | }
    | |_- in this macro invocation
    |
    |
    = help: mark blocks that do not contain Rust code as text: 