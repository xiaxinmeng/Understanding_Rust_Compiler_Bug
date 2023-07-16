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
* 632 error codes
* highest error code: E0790
tidy error: /checkout/src/test/ui/type/issue-101208.rs: too many trailing newlines (2)
Found 0 error(s) in error codes
Done!
Done!
tidy error: /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs: too many lines (3027) (add `// ignore-tidy-filelength` to the file to suppress this error)
some tidy checks failed
Build completed unsuccessfully in 0:00:10
