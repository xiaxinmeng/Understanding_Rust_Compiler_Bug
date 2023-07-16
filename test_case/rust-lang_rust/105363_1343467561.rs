
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_index/src/lib.rs:29:32
    |
27  | macro_rules! static_assert_size {
    | ------------------------------- in this expansion of `rustc_data_structures::static_assert_size!`
28  |     ($ty:ty, $size:expr) => {
29  |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 24 elements, found one with 16 elements
    |
   ::: compiler/rustc_parse_format/src/lib.rs:913:1
    |
913 | rustc_data_structures::static_assert_size!(Piece<'_>, 24);
    | --------------------------------------------------------- in this macro invocation
