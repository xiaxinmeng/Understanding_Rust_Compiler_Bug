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
* 632 error codes
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
tidy error: /checkout/src/test/ui/const-generics/bool_cond.rs: too many trailing newlines (2)
Done!
tidy error: /checkout/compiler/rustc_feature/src/active.rs:425: no tracking issue for feature impl_unified_exhaustive_const_traits
some tidy checks failed
