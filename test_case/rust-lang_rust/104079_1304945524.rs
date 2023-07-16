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
---- src/vec/mod.rs - vec::Vec<T,A>::push_with_ref (line 1851) stdout ----
error[E0658]: use of unstable library feature 'vec_push_with_ref'
 --> src/vec/mod.rs:1853:19
  |
5 | let int_ref = vec.push_with_ref(3);
  |
  = note: see issue #104075 <https://github.com/rust-lang/rust/issues/104075> for more information
  = note: see issue #104075 <https://github.com/rust-lang/rust/issues/104075> for more information
  = help: add `#![feature(vec_push_with_ref)]` to the crate attributes to enable
error[E0308]: mismatched types
 --> src/vec/mod.rs:1854:1
  |
  |
6 | assert_eq!(vec.last(), int_ref);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Option`, found `&{integer}`
  = note:   expected enum `Option<&{integer}>`
          found reference `&{integer}`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: try wrapping the expression in `Some`
help: try wrapping the expression in `Some`
 --> /checkout/library/core/src/macros/mod.rs:40:35
  |
40|                 if !(*left_val == Some(*right_val)) {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0658.
---
    src/vec/mod.rs - vec::Vec<T,A>::push_with_ref (line 1851)

test result: FAILED. 656 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 10.01s

error: doctest failed, to rerun pass `-p alloc --doc`
