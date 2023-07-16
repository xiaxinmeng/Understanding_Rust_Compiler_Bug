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
tidy error: /checkout/src/doc/rustdoc/src/SUMMARY.md:7: tab character
tidy error: /checkout/src/doc/rustdoc/src/SUMMARY.md:8: tab character
tidy error: /checkout/src/doc/rustdoc/src/SUMMARY.md:9: tab character
tidy error: /checkout/src/doc/rustdoc/src/SUMMARY.md:10: tab character
tidy error: /checkout/src/doc/rustdoc/src/advanced-features.md: missing trailing newline
tidy error: /checkout/src/doc/rustdoc/src/command-line-arguments.md: missing trailing newline
some tidy checks failed
Build completed unsuccessfully in 0:00:10
