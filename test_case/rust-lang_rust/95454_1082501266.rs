plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/compiler/rustc_data_structures/src/profiling.rs:653: tab character
tidy error: /checkout/compiler/rustc_data_structures/src/profiling.rs:653: trailing whitespace
tidy error: /checkout/compiler/rustc_data_structures/src/profiling.rs:654: tab character
tidy error: /checkout/compiler/rustc_data_structures/src/profiling.rs:655: tab character
tidy error: /checkout/compiler/rustc_data_structures/src/profiling.rs:656: tab character
some tidy checks failed
Build completed unsuccessfully in 0:00:11
