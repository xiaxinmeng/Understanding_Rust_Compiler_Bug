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
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/compiler/rustc_feature/src/active.rs:400: feature generator_clone is not sorted by feature name (should be between fn_align and generators)
tidy error: /checkout/compiler/rustc_feature/src/active.rs:400: no tracking issue for feature generator_clone
Expected a gate test for the feature 'generator_clone'.
tidy error: Found 1 features without a gate test.
Hint: create a failing test file named 'feature-gate-generator_clone.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(generator_clone)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-generator_clone line to the test file.
some tidy checks failed
