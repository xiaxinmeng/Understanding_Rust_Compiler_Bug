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
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 629 error codes
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes/E0667.md:11: line longer than 80 chars
Found 504 error codes
Found 0 error(s) in error codes
Done!
* 371 features
