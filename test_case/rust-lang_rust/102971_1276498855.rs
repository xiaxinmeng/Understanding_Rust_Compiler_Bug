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
tidy error: /checkout/compiler/rustc_feature/src/accepted.rs:173 feature half_open_range_patterns already specified with level 'unstable'
tidy error: Found 1 features without a gate test.
Expected a gate test for the feature 'half_open_range_patterns'.
Hint: create a failing test file named 'feature-gate-half_open_range_patterns.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(half_open_range_patterns)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-half_open_range_patterns line to the test file.
some tidy checks failed
