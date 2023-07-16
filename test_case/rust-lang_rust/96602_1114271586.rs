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
tidy error: /checkout/src/bootstrap/bootstrap.py:113: line longer than 100 chars
tidy error: /checkout/src/bootstrap/bootstrap.py:126: line longer than 100 chars
tidy error: /checkout/src/bootstrap/bootstrap.py:147: trailing whitespace
some tidy checks failed
Build completed unsuccessfully in 0:00:12
