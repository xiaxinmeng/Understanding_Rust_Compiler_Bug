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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 629 error codes
* highest error code: E0787
tidy error: /checkout/compiler/rustc_codegen_llvm/src/attributes.rs:63: TODO is deprecated; use FIXME
Found 0 error(s) in error codes
Done!
Done!
tidy error: /checkout/compiler/rustc_llvm/llvm-wrapper/RustWrapper.cpp:342: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:00:10
