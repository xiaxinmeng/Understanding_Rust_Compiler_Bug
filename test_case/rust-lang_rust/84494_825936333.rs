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
    Finished release [optimized] target(s) in 11.02s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: /checkout/src/librustdoc/clean/types.rs:650: trailing whitespace
tidy error: /checkout/src/librustdoc/clean/types.rs:652: trailing whitespace
tidy error: /checkout/src/librustdoc/clean/types.rs:654: trailing whitespace
tidy error: /checkout/src/librustdoc/clean/types.rs:662: trailing whitespace
tidy error: /checkout/src/librustdoc/clean/types.rs:1103: trailing whitespace
tidy error: /checkout/src/librustdoc/clean/types.rs:1107: trailing whitespace
tidy error: /checkout/src/librustdoc/clean/types.rs:1111: trailing whitespace
tidy error: /checkout/src/librustdoc/doctest.rs:1096: trailing whitespace
* highest error code: E0781
tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links.rs:797: trailing whitespace
Found 516 error codes
Found 0 error codes with no tests
Found 0 error codes with no tests
Done!
* 326 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:13
