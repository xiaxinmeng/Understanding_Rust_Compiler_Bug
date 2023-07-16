plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 31 elements, found one with 32 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    |
   ::: src/librustdoc/clean/types.rs:922:1
   ::: src/librustdoc/clean/types.rs:922:1
    |
922 |   rustc_data_structures::static_assert_size!(DocFragment, 31);


error[E0277]: can't compare `&types::DocFragment` with `std::option::Option<&types::DocFragment>`
    --> src/librustdoc/clean/types.rs:1092:25
     |
1092 |             if new_frag == self.doc_strings.last() {
     |                         ^^ no implementation for `&types::DocFragment == std::option::Option<&types::DocFragment>`
     |
     = help: the trait `std::cmp::PartialEq<std::option::Option<&types::DocFragment>>` is not implemented for `&types::DocFragment`
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:03:00
