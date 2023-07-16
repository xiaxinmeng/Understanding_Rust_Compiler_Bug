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
* highest error code: E0791
Found 507 error codes
Found 0 error(s) in error codes
Done!
thread '<unnamed>' panicked at 'Encountered error while testing Git status: "failed to execute git log command: fatal: ambiguous argument 'origin/master..HEAD': unknown revision or path not in the working tree.\nUse '--' to separate paths from revisions, like this:\n'git <command> [<revision>...] -- [<file>...]'\n"', src/tools/tidy/src/no_merge.rs:20:25
* 392 features
* 392 features
thread 'main' panicked at 'a scoped thread panicked', src/tools/tidy/src/main.rs:38:5
