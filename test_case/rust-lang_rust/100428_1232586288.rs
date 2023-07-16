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
tidy error: The stabilization version 1.65.0 of lib feature `generic_nonzero_new` is written out but should be CURRENT_RUSTC_VERSION
tidy error: The stabilization version 1.65.0 of lib feature `from_generic_nonzero` is written out but should be CURRENT_RUSTC_VERSION
tidy error: The stabilization version 1.65.0 of lib feature `generic_nonzero_get` is written out but should be CURRENT_RUSTC_VERSION
tidy error: The stabilization version 1.65.0 of lib feature `generic_nonzero_bitor` is written out but should be CURRENT_RUSTC_VERSION
tidy error: The stabilization version 1.65.0 of lib feature `generic_nonzero_fmt` is written out but should be CURRENT_RUSTC_VERSION
some tidy checks failed
