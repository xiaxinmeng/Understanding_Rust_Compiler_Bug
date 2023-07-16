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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 625 error codes
* highest error code: E0783
tidy error: /checkout/src/test/ui/feature-gates/where_eq_pred.rs: too many trailing newlines (2)
Found 0 error codes with no tests
Done!
Done!
tidy error: /checkout/compiler/rustc_feature/src/active.rs:688: no tracking issue for feature where_pred_equality
tidy error: Found 1 features without a gate test.
Expected a gate test for the feature 'where_pred_equality'.
Hint: create a failing test file named 'feature-gate-where_pred_equality.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(where_pred_equality)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-where_pred_equality line to the test file.
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
