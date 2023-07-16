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
Found 517 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/test/debuginfo/fix-sized-array.rs: missing trailing newline
tidy error: /checkout/src/test/debuginfo/result-types.rs: missing trailing newline
tidy error: /checkout/src/test/debuginfo/rc_arc.rs:38: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/rwlock-read.rs: missing trailing newline
tidy error: /checkout/src/test/debuginfo/mutable-locs.rs: missing trailing newline
tidy error: /checkout/src/test/debuginfo/mutex.rs:24: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/mutex.rs: missing trailing newline
tidy error: /checkout/src/test/debuginfo/range-types.rs: missing trailing newline
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:13
