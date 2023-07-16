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
tidy error: /checkout/compiler/rustc_feature/src/active.rs:494: no tracking issue for feature with_negative_coherence
tidy error: /checkout/compiler/rustc_feature/src/active.rs:496: feature specialization is not sorted by feature name (should be between simd_ffi and with_negative_coherence)
tidy error: Found 1 features without a gate test.
Expected a gate test for the feature 'with_negative_coherence'.
Hint: create a failing test file named 'feature-gate-with_negative_coherence.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(with_negative_coherence)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-with_negative_coherence line to the test file.
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
