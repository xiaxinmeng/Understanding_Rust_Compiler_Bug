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
* highest error code: E0783
Found 499 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/test/ui/rfc-2091-track-caller/tracked-closure.rs:14: line longer than 100 chars
tidy error: /checkout/src/test/ui/rfc-2091-track-caller/tracked-closure.rs:19: line longer than 100 chars
tidy error: /checkout/src/test/ui/rfc-2091-track-caller/tracked-closure.rs:24: line longer than 100 chars
tidy error: /checkout/src/test/ui/rfc-2091-track-caller/tracked-closure.rs:29: line longer than 100 chars
tidy error: /checkout/src/test/ui/rfc-2091-track-caller/tracked-closure.rs:87: line longer than 100 chars
tidy error: /checkout/src/test/ui/rfc-2091-track-caller/tracked-closure.rs:95: line longer than 100 chars
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:11
