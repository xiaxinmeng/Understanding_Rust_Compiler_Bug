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
tidy error: /checkout/compiler/rustc_resolve/src/lib.rs:2024: trailing whitespace
tidy error: /checkout/compiler/rustc_resolve/src/access_levels.rs:192: trailing whitespace
tidy error: /checkout/compiler/rustc_resolve/src/imports.rs:1103: trailing whitespace
some tidy checks failed
Build completed unsuccessfully in 0:00:16
