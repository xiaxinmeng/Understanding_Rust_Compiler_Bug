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
     |
1119 |         ignore_reasons,
     |         ^^^^^^^^^^^^^^ `TestDesc` does not have this field
     |
     = note: available fields are: `name`, `ignore`, `ignore_message`, `should_panic`, `compile_fail` ... and 2 others
For more information about this error, try `rustc --explain E0560`.
error: could not compile `compiletest` due to previous error
Build completed unsuccessfully in 0:00:58
