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
* 623 error codes
* highest error code: E0781
tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links/early.rs:19: tab character
tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links/early.rs:20: tab character
tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links/early.rs:21: tab character
tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links/early.rs:22: tab character
Found 0 error codes with no tests
Done!
* 326 features
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:13
