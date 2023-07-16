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
tidy error: invalid license `(MIT OR Apache-2.0) AND Unicode-DFS-2016` in `unicode-ident 1.0.5 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: Dependencies for rustc not explicitly permitted:
* unicode-ident 1.0.5 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed
Build completed unsuccessfully in 0:00:09
