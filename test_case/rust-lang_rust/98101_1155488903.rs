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
Found 505 error codes
Found 0 error(s) in error codes
Done!
* 378 features
tidy error: /checkout/library/std/src/os/watchos/fs.rs: missing trailing newline
tidy error: /checkout/library/std/src/os/watchos/raw.rs: missing trailing newline
tidy error: /checkout/library/std/src/os/watchos/mod.rs: missing trailing newline
some tidy checks failed
