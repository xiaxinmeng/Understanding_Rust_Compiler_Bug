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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 633 error codes
* highest error code: E0791
thread '<unnamed>' panicked at 'Encountered error while testing Git status: "failed to execute git log command: fatal: ambiguous argument 'origin/master..HEAD': unknown revision or path not in the working tree.\nUse '--' to separate paths from revisions, like this:\n'git <command> [<revision>...] -- [<file>...]'\n"', src/tools/tidy/src/no_merge.rs:20:25
Found 507 error codes
Found 0 error(s) in error codes
Done!
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', src/tools/tidy/src/main.rs:42:61
