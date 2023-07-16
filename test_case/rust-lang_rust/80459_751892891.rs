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
Checking which error codes lack tests...
Found 434 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/compiler/rustc_feature/src/active.rs:624: no tracking issue for feature edition_macro_pats
tidy error: Found 1 features without a gate test.
Expected a gate test for the feature 'edition_macro_pats'.
Hint: create a failing test file named 'feature-gate-edition_macro_pats.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(edition_macro_pats)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-edition_macro_pats line to the test file.
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

