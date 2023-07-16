plain
   Compiling rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0308]: mismatched types
  --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
   |
3  | / macro_rules! static_assert_size {
4  | |     ($ty:ty, $size:expr) => {
5  | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
   | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 24 elements, found one with 16 elements
7  | | }
   | |_- in this expansion of `rustc_data_structures::static_assert_size!`
   |
  ::: compiler/rustc_errors/src/lib.rs:62:1
  ::: compiler/rustc_errors/src/lib.rs:62:1
   |
62 |   rustc_data_structures::static_assert_size!(PResult<'_, bool>, 24);

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_errors` due to previous error
warning: build failed, waiting for other jobs to finish...
