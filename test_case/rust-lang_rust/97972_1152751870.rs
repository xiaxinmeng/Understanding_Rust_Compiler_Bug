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
tidy error: /checkout/src/doc/rustdoc/src/write-documentation/the-doc-attribute.md:90: trailing whitespace
tidy error: /checkout/src/doc/rustdoc/src/write-documentation/the-doc-attribute.md:92: trailing whitespace
some tidy checks failed
Build completed unsuccessfully in 0:00:11
