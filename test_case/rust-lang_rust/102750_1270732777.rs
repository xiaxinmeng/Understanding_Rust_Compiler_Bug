plain
   Compiling rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_index/src/lib.rs:21:32
     |
19   | macro_rules! static_assert_size {
     | ------------------------------- in this expansion of `static_assert_size!`
20   |     ($ty:ty, $size:expr) => {
21   |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 88 elements, found one with 80 elements
    ::: compiler/rustc_hir/src/hir.rs:3543:5
     |
     |
3543 |     static_assert_size!(TraitItem<'_>, 88);

error[E0308]: mismatched types
    --> /checkout/compiler/rustc_index/src/lib.rs:21:32
     |
     |
19   | macro_rules! static_assert_size {
     | ------------------------------- in this expansion of `static_assert_size!`
20   |     ($ty:ty, $size:expr) => {
21   |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 48 elements, found one with 40 elements
    ::: compiler/rustc_hir/src/hir.rs:3544:5
     |
     |
3544 |     static_assert_size!(TraitItemKind<'_>, 48);

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_hir` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
