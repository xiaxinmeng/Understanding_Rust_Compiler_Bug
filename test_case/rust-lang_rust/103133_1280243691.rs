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
tidy error: /checkout/library/core/src/fmt/num.rs:397: undocumented unsafe
tidy error: /checkout/library/core/src/fmt/num.rs:400: undocumented unsafe
tidy error: /checkout/library/core/src/mem/maybe_uninit.rs:1309: TODO is deprecated; use FIXME
some tidy checks failed
Build completed unsuccessfully in 0:00:09
