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
tidy error: /checkout/compiler/rustc_codegen_llvm/src/attributes.rs:64: trailing whitespace
Found 504 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/compiler/rustc_llvm/llvm-wrapper/RustWrapper.cpp:342: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:00:11
