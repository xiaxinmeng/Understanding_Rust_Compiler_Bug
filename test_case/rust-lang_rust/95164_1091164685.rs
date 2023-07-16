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
    Finished release [optimized] target(s) in 8.70s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: /checkout/src/bootstrap/bootstrap.py:987: line longer than 100 chars
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
Done!
