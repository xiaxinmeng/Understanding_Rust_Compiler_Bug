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
    Finished release [optimized] target(s) in 8.31s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: duplicate error code: 725
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes.rs:425: E0725: include_str!("./error_codes/E0725.md"),
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes.rs:426: E0725: include_str!("./error_codes/E0726.md"),
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes/E0726.md:13: line longer than 80 chars
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes/E0726.md:15: trailing whitespace
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes/E0726.md:19: line longer than 80 chars
`/checkout/compiler/rustc_error_codes/src/error_codes/E0726.md` doesn't use its own error code in compile_fail example
Done!
* 345 features
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:10
