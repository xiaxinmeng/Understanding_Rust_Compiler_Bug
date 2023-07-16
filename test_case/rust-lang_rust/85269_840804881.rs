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
tidy error: /checkout/src/test/debuginfo/type-names.rs:172: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:187: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:188: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:193: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:198: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:200: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:206: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:209: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:223: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:224: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:225: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:226: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:229: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:233: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:234: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:235: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:240: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/type-names.rs:243: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/generic-struct.rs:59: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/function-names.rs:43: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/function-names.rs:44: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/function-names.rs:45: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/function-names.rs:46: line longer than 100 chars
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:14
