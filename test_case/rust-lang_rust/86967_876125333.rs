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
* highest error code: E0783
Found 499 error codes
Found 0 error codes with no tests
Done!
tidy error: Found 1 features without a gate test.
Expected a gate test for the feature 'capture_disjoint_fields'.
Hint: create a failing test file named 'feature-gate-capture_disjoint_fields.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(capture_disjoint_fields)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-capture_disjoint_fields line to the test file.
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
