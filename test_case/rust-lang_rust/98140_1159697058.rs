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
* highest error code: E0788
Found 505 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/src/test/ui/numbers-arithmetic/promoted_overflow.rs:6: line longer than 100 chars
tidy error: /checkout/src/test/ui/panics/unique-panic.rs:3: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:00:12
