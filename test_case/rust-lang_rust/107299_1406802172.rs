plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
error[E0658]: use of unstable library feature 'structural_match'
  --> compiler/rustc_index/src/vec/tests.rs:6:1
   |
6  | / rustc_macros::newtype_index! {
7  | |     #[max = 0xFFFF_FFFA]
8  | |     struct MyIdx {}
   | | ^ in this macro invocation
   | |_|
   | 
   |
   |
  ::: /checkout/compiler/rustc_macros/src/lib.rs:53:1
   |
53 |   pub fn newtype_index(input: TokenStream) -> TokenStream {
   |
   = note: see issue #31434 <https://github.com/rust-lang/rust/issues/31434> for more information
   = note: see issue #31434 <https://github.com/rust-lang/rust/issues/31434> for more information
   = help: add `#![feature(structural_match)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `rustc_index` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:49
