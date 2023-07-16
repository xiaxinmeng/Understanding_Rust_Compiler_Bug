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
Found 435 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/test/ui/ufcs/ufcs-qpath-missing-params.rs:18: line longer than 100 chars
tidy error: /checkout/src/test/ui/const-generics/issues/issue-76595.rs:16: trailing whitespace
tidy error: /checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs:7: trailing whitespace
tidy error: /checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs:13: trailing whitespace
tidy error: /checkout/src/test/ui/issues/issue-53251.rs: ignoring line length unnecessarily
tidy error: /checkout/src/test/ui/traits/test-2.rs:10: line longer than 100 chars
tidy error: /checkout/src/test/ui/traits/test-2.rs:12: line longer than 100 chars
tidy error: /checkout/compiler/rustc_resolve/src/late/lifetimes.rs: too many lines (3008) (add `// ignore-tidy-filelength` to the file to suppress this error)
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:16
