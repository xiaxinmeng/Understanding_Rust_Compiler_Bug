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
    Finished release [optimized] target(s) in 10.31s
tidy check
Checking which error codes lack tests...
tidy: Skipping binary file check, read-only filesystem
tidy error: /checkout/compiler/rustc_lint/src/builtin.rs: too many lines (3052) (add `// ignore-tidy-filelength` to the file to suppress this error)
* highest error code: E0781
Found 516 error codes
Found 0 error codes with no tests
Done!
Done!
* 325 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:12
