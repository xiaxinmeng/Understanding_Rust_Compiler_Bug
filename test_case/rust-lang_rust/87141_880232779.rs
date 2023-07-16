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
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/pattern/issue-71042-opaquely-typed-constant-used-in-pattern.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/impl-trait/bindings.stderr"
tidy error: Stray file with UI testing output: "/checkout/src/test/ui/feature-gates/feature-gate-impl_trait_in_bindings.stderr"
Found 0 error codes with no tests
Done!
Done!
tidy error: Found 1 features without a gate test.
Expected a gate test for the feature 'impl_trait_in_bindings'.
Hint: create a failing test file named 'feature-gate-impl_trait_in_bindings.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(impl_trait_in_bindings)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-impl_trait_in_bindings line to the test file.
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:14
