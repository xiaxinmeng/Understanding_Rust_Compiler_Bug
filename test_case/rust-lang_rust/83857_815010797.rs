plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0781
Found 516 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/test/rustdoc-ui/no-run-flag.rs:14: unexplained "