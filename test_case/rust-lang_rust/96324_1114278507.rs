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
Found 504 error codes
Found 0 error(s) in error codes
Done!
* 374 features
tidy error: /checkout/library/std/src/os/linux/net.rs:54: trailing whitespace
tidy error: /checkout/library/std/src/os/linux/net.rs:62: trailing whitespace
tidy error: /checkout/library/std/src/os/linux/net.rs: missing trailing newline
some tidy checks failed
