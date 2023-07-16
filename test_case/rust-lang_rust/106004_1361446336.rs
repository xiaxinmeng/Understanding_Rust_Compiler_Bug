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
tidy error: Found 1 features without a gate test.
tidy error: The stabilization version 1.68.0 of lang feature `const_closure` is written out but should be CURRENT_RUSTC_VERSION
Expected a gate test for the feature 'const_closure'.
Hint: create a failing test file named 'feature-gate-const_closure.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_closure)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_closure line to the test file.
some tidy checks failed
