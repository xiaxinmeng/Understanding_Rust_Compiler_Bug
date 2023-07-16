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
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/src/test/ui/const-generics/bool_cond.rs: too many trailing newlines (2)
tidy error: /checkout/src/test/ui/const-generics/bool_cond_fn.rs: too many trailing newlines (2)
tidy error: /checkout/compiler/rustc_feature/src/active.rs:425: no tracking issue for feature impl_unified_exhaustive_const_traits
tidy error: Found 1 features without a gate test.
Expected a gate test for the feature 'impl_unified_exhaustive_const_traits'.
Hint: create a failing test file named 'feature-gate-impl_unified_exhaustive_const_traits.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(impl_unified_exhaustive_const_traits)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-impl_unified_exhaustive_const_traits line to the test file.
some tidy checks failed
