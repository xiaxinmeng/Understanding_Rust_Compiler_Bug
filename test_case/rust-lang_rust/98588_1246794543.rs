plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c97922dca563cb7f9385b18dbf7ca8c97f8e1597 and f6a22a741bda251953b59fdf43bb91f01da48d5a
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_index/src/lib.rs:23:32
    |
21  | macro_rules! static_assert_size {
    | ------------------------------- in this expansion of `static_assert_size!`
22  |     ($ty:ty, $size:expr) => {
23  |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 64 elements, found one with 72 elements
   ::: compiler/rustc_middle/src/thir.rs:856:5
    |
    |
856 |     static_assert_size!(Pat<'_>, 64);

error[E0308]: mismatched types
   --> /checkout/compiler/rustc_index/src/lib.rs:23:32
    |
    |
21  | macro_rules! static_assert_size {
    | ------------------------------- in this expansion of `static_assert_size!`
22  |     ($ty:ty, $size:expr) => {
23  |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 48 elements, found one with 56 elements
   ::: compiler/rustc_middle/src/thir.rs:858:5
    |
    |
858 |     static_assert_size!(PatKind<'_>, 48);

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
