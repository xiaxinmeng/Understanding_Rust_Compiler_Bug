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
Found 503 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/compiler/rustc_feature/src/active.rs:537: no tracking issue for feature used_with_arg
tidy error: /checkout/compiler/rustc_typeck/src/collect.rs: too many lines (3003) (add `// ignore-tidy-filelength` to the file to suppress this error)
tidy error: Found 1 features without a gate test.
Expected a gate test for the feature 'used_with_arg'.
Hint: create a failing test file named 'feature-gate-used_with_arg.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(used_with_arg)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-used_with_arg line to the test file.
some tidy checks failed
