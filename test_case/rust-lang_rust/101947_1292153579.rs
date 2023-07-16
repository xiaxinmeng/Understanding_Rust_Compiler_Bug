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
tidy error: /checkout/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1214: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_hir_typeck/src/closure.rs:607: TODO is deprecated; use FIXME
tidy error: /checkout/src/test/ui/tests2/tto1.rs:58: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_hir_typeck/src/method/confirm.rs:413: TODO is deprecated; use FIXME
some tidy checks failed
Build completed unsuccessfully in 0:00:11
