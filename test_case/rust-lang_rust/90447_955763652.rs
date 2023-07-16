plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 104 elements, found one with 96 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    |
   ::: src/librustdoc/clean/types.rs:128:1
   ::: src/librustdoc/clean/types.rs:128:1
    |
128 |   rustc_data_structures::static_assert_size!(Crate, 104);

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:00
