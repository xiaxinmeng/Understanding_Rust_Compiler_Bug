plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_index/src/lib.rs:21:32
    |
19  | / macro_rules! static_assert_size {
20  | |     ($ty:ty, $size:expr) => {
21  | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 24 elements, found one with 16 elements
23  | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    |
   ::: compiler/rustc_middle/src/mir/query.rs:345:1
   ::: compiler/rustc_middle/src/mir/query.rs:345:1
    |
345 |   rustc_data_structures::static_assert_size!(ConstraintCategory<'_>, 24);

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
