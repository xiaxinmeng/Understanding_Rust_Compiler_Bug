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
tidy error: /checkout/library/core/src/array/mod.rs:316: XXX is deprecated; use FIXME
tidy error: /checkout/library/core/src/array/mod.rs:345: XXX is deprecated; use FIXME
tidy error: The stabilization version 1.666.0 of lib feature `slice_try_as_chunks` is newer than the current 1.67.0
some tidy checks failed
