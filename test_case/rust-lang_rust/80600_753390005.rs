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
tidy error: /checkout/library/core/src/mem/maybe_uninit.rs:807: trailing whitespace
tidy error: /checkout/library/core/src/mem/maybe_uninit.rs:808: trailing whitespace
tidy error: /checkout/library/core/src/mem/maybe_uninit.rs:810: trailing whitespace
tidy error: /checkout/library/core/src/mem/maybe_uninit.rs:812: trailing whitespace
tidy error: /checkout/library/core/src/mem/maybe_uninit.rs:820: trailing whitespace
tidy error: /checkout/library/core/src/mem/maybe_uninit.rs:825: trailing whitespace
tidy error: /checkout/library/core/src/mem/maybe_uninit.rs:830: trailing whitespace
tidy error: /checkout/library/core/tests/mem.rs:155: trailing whitespace
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

