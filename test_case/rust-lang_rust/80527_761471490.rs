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
tidy error: /checkout/src/librustdoc/lint.rs:67: tab character
tidy error: /checkout/src/librustdoc/lint.rs:68: tab character
tidy error: /checkout/src/librustdoc/lint.rs:69: tab character
tidy error: /checkout/src/librustdoc/lint.rs:70: tab character
tidy error: /checkout/src/librustdoc/lint.rs:71: tab character
tidy error: /checkout/compiler/rustc_lint/src/lib.rs:72: trailing whitespace
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

