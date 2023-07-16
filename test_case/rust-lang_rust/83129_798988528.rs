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
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:38: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:54: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:182: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:184: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:186: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:188: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:190: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:192: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:194: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:196: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:198: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:200: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:202: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_mir_build/src/check_unsafety.rs:204: TODO is deprecated; use FIXME
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:14
