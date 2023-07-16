plain
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_index/src/lib.rs:29:32
     |
27   | macro_rules! static_assert_size {
     | ------------------------------- in this expansion of `static_assert_size!`
28   |     ($ty:ty, $size:expr) => {
29   |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 136 elements, found one with 144 elements
    ::: compiler/rustc_ast/src/ast.rs:3144:31
     |
     |
3144 |     static_assert_size!(Item, 136);
     |     |                         |
     |     |                         |
     |     |                         help: consider specifying the actual array length: `144`

error[E0308]: mismatched types
    --> /checkout/compiler/rustc_index/src/lib.rs:29:32
     |
     |
27   | macro_rules! static_assert_size {
     | ------------------------------- in this expansion of `static_assert_size!`
28   |     ($ty:ty, $size:expr) => {
29   |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 64 elements, found one with 72 elements
    ::: compiler/rustc_ast/src/ast.rs:3145:35
     |
     |
3145 |     static_assert_size!(ItemKind, 64);
     |     |                             |
     |     |                             |
     |     |                             help: consider specifying the actual array length: `72`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_ast` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
