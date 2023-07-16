plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_index/src/lib.rs:21:32
    |
19  | / macro_rules! static_assert_size {
20  | |     ($ty:ty, $size:expr) => {
21  | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 72 elements, found one with 64 elements
23  | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    |
   ::: src/librustdoc/clean/types.rs:126:1
   ::: src/librustdoc/clean/types.rs:126:1
    |
126 |   rustc_data_structures::static_assert_size!(Crate, 72);

error[E0308]: mismatched types
    --> /checkout/compiler/rustc_index/src/lib.rs:21:32
     |
     |
19   | / macro_rules! static_assert_size {
20   | |     ($ty:ty, $size:expr) => {
21   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 64 elements, found one with 72 elements
23   | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     |
    ::: src/librustdoc/clean/types.rs:1583:1
