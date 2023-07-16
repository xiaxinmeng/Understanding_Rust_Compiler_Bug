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
tidy error: library/core/src/primitive_docs.rs and library/std/src/primitive_docs.rs have different contents
Checking which error codes lack tests...
* 627 error codes
* highest error code: E0785
tidy error: Empty file with UI testing output: "/checkout/src/test/ui/type-alias-impl-trait/different_defining_uses_never_type.full_tait.stderr"
Found 0 error codes with no tests
Done!
* 351 features
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:10
