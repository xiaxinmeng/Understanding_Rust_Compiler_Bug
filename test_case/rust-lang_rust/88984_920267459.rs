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
* highest error code: E0785
Found 501 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/compiler/rustc_type_ir/src/lib.rs:243: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_type_ir/src/lib.rs:290: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_type_ir/src/lib.rs:337: TODO is deprecated; use FIXME
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
