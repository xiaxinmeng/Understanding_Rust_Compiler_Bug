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
tidy error: /checkout/src/test/run-make/raw-dylib-import-name-type/extern.c: too many trailing newlines (2)
Found 0 error(s) in error codes
Done!
Done!
tidy error: /checkout/src/test/ui/rfc-2627-raw-dylib/import-name-type-multiple.rs:6: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:00:11
