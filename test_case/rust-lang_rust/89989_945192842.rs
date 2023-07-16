plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
    --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
     |
3    | / macro_rules! static_assert_size {
4    | |     ($ty:ty, $size:expr) => {
5    | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 64 elements, found one with 48 elements
7    | | }
     | |_- in this expansion of `rustc_data_structures::static_assert_size!`
     |
    ::: src/librustdoc/clean/types.rs:1455:1
