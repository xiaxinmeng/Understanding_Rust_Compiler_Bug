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
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
Done!
tidy error: Dependencies for cranelift not explicitly permitted:
* arrayvec 0.7.2 (registry+https://github.com/rust-lang/crates.io-index)
* bumpalo 3.11.0 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed
Build completed unsuccessfully in 0:00:11
