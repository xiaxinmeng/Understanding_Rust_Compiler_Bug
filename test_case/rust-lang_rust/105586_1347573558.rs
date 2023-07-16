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
* highest error code: E0791
Found 507 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/compiler/rustc_feature/src/active.rs:435: feature interoperable_abi is not sorted by feature name (should be between inline_const_pat and intra_doc_pointers)
tidy error: /checkout/compiler/rustc_feature/src/active.rs:435: no tracking issue for feature interoperable_abi
tidy error: Found 1 features without a gate test.
Expected a gate test for the feature 'interoperable_abi'.
Hint: create a failing test file named 'feature-gate-interoperable_abi.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(interoperable_abi)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-interoperable_abi line to the test file.
some tidy checks failed
