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
tidy error: /checkout/library/core/src/num/nonzero.rs:303: undocumented unsafe
tidy error: /checkout/library/core/src/num/nonzero.rs:345: undocumented unsafe
tidy error: /checkout/library/core/src/num/nonzero.rs:386: undocumented unsafe
tidy error: /checkout/library/core/tests/nonzero.rs: too many trailing newlines (2)
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:15
