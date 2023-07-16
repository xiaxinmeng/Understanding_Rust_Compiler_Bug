plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/compiler/rustc_typeck/src/collect.rs:98: trailing whitespace
tidy error: /checkout/compiler/rustc_typeck/src/collect.rs:105: trailing whitespace
tidy error: /checkout/compiler/rustc_typeck/src/collect.rs:108: trailing whitespace
tidy error: /checkout/compiler/rustc_typeck/src/collect.rs:111: trailing whitespace
tidy error: /checkout/compiler/rustc_typeck/src/collect.rs:113: trailing whitespace
tidy error: /checkout/compiler/rustc_typeck/src/check/fn_ctxt/mod.rs:35: trailing whitespace
tidy error: /checkout/compiler/rustc_typeck/src/check/fn_ctxt/mod.rs:37: trailing whitespace
some tidy checks failed
Build completed unsuccessfully in 0:00:12
