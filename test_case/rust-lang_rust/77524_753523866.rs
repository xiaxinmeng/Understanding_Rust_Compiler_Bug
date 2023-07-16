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
tidy error: /checkout/src/test/ui/async-await/issues/issue-65159.rs:5: line longer than 100 chars
tidy error: /checkout/src/test/ui/traits/trait-test-2.rs: ignoring line length unnecessarily
tidy error: /checkout/src/test/ui/seq-args.rs:4: line longer than 100 chars
tidy error: /checkout/src/test/ui/seq-args.rs:7: line longer than 100 chars
tidy error: /checkout/src/test/ui/const-generics/incorrect-number-of-const-args.rs:11: line longer than 100 chars
tidy error: /checkout/src/test/ui/const-generics/incorrect-number-of-const-args.rs:12: line longer than 100 chars
tidy error: /checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs:6: line longer than 100 chars
tidy error: /checkout/src/test/ui/methods/method-call-lifetime-args-fail.rs:17: line longer than 100 chars
tidy error: /checkout/src/test/ui/methods/method-call-lifetime-args-fail.rs:19: line longer than 100 chars
tidy error: /checkout/src/test/ui/methods/method-call-lifetime-args-fail.rs:64: line longer than 100 chars
tidy error: /checkout/src/test/ui/methods/method-call-lifetime-args-fail.rs:66: line longer than 100 chars
tidy error: /checkout/src/test/ui/issues/issue-18423.rs:4: line longer than 100 chars
tidy error: /checkout/src/test/ui/issues/issue-53251.rs:12: line longer than 100 chars
tidy error: /checkout/src/test/ui/issues/issue-53251.rs:13: line longer than 100 chars
tidy error: /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:48: line longer than 100 chars
tidy error: /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:54: line longer than 100 chars
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

