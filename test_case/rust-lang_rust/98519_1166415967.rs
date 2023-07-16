plain
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_index/src/lib.rs:21:32
     |
19   | / macro_rules! static_assert_size {
20   | |     ($ty:ty, $size:expr) => {
21   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 80 elements, found one with 88 elements
23   | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     |
    ::: compiler/rustc_hir/src/hir.rs:3495:5
    ::: compiler/rustc_hir/src/hir.rs:3495:5
     |
3495 |       rustc_data_structures::static_assert_size!(super::Item<'static>, 80);

error[E0308]: mismatched types
    --> /checkout/compiler/rustc_index/src/lib.rs:21:32
     |
     |
19   | / macro_rules! static_assert_size {
20   | |     ($ty:ty, $size:expr) => {
21   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 88 elements, found one with 96 elements
23   | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     |
    ::: compiler/rustc_hir/src/hir.rs:3496:5
    ::: compiler/rustc_hir/src/hir.rs:3496:5
     |
3496 |       rustc_data_structures::static_assert_size!(super::TraitItem<'static>, 88);

error[E0308]: mismatched types
    --> /checkout/compiler/rustc_index/src/lib.rs:21:32
     |
     |
19   | / macro_rules! static_assert_size {
20   | |     ($ty:ty, $size:expr) => {
21   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 80 elements, found one with 88 elements
23   | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     |
    ::: compiler/rustc_hir/src/hir.rs:3497:5
    ::: compiler/rustc_hir/src/hir.rs:3497:5
     |
3497 |       rustc_data_structures::static_assert_size!(super::ImplItem<'static>, 80);

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_hir` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
