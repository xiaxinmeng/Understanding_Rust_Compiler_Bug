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
tidy error: Dependencies not explicitly permitted:
* bitmaps 2.1.0 (registry+https://github.com/rust-lang/crates.io-index)
* im-rc 15.0.0 (registry+https://github.com/rust-lang/crates.io-index)
* sized-chunks 0.6.4 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed
Build completed unsuccessfully in 0:00:12
