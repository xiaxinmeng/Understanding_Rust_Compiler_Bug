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
tidy error: /checkout/library/core/src/num/uint_macros.rs:699: undocumented unsafe
tidy error: /checkout/library/core/src/num/int_macros.rs:1759: undocumented unsafe
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

