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
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:10: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:11: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:13: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:14: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:17: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:18: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:20: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:21: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:25: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:27: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:28: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:32: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:34: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:35: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:39: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:41: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:42: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:46: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:48: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:49: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:53: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:55: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:56: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:71: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:72: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:74: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs:75: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/msvc-pretty-enums.rs: missing trailing newline
tidy error: /checkout/src/test/debuginfo/pretty-std.rs:118: line longer than 100 chars
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:14
