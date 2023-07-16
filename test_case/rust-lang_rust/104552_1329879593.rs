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
thread '<unnamed>' panicked at 'not yet implemented', src/tools/tidy/src/x_version.rs:7:20
* 392 features
* 392 features
thread 'main' panicked at 'a scoped thread panicked', src/tools/tidy/src/main.rs:38:5
