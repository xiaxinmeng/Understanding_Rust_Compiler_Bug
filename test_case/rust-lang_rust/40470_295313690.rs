
error: use of unstable library feature 'float_bits_conv': recently added (see issue #40470)
cargo/registry/src/github.com-1ecc6299db9ec823/ieee754-0.2.1/src/lib.rs:458:17
    |
458 |                 Self::from_bits(
    |                 ^^^^^^^^^^^^^^^
...
610 | mk_impl!(f64, u64, i16, u16, u64, 11, 52);
    | ------------------------------------------ in this macro invocation
