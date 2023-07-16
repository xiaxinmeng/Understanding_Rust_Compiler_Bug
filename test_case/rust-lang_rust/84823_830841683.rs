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
tidy error: /checkout/library/core/src/fmt/mod.rs:1266: undocumented unsafe
Found 515 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/compiler/rustc_builtin_macros/src/format.rs:63: XXX is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_builtin_macros/src/format.rs:365: XXX is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_builtin_macros/src/format.rs:428: XXX is deprecated; use FIXME
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:12
