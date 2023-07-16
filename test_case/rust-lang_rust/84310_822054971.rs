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
* highest error code: E0781
Found 516 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/compiler/rustc_feature/src/active.rs:586: feature cmse_nonsecure_entry is not sorted by "since" (version number)
tidy error: Found 2 features without a gate test.
Expected a gate test for the feature 'const_fn_trait_bound'.
Hint: create a failing test file named 'feature-gate-const_fn_trait_bound.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_fn_trait_bound)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_fn_trait_bound line to the test file.
Expected a gate test for the feature 'const_fn_unsize'.
Hint: create a failing test file named 'feature-gate-const_fn_unsize.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_fn_unsize)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_fn_unsize line to the test file.
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:12
