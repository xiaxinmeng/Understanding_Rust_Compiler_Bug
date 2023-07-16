plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: the feature `const_trait_impl` is incomplete and may not be safe to use and/or cause compiler crashes
   |
25 | #![feature(const_trait_impl)]
   |            ^^^^^^^^^^^^^^^^
   |
   |
   = note: `-D incomplete-features` implied by `-D warnings`

error: aborting due to previous error

error: could not compile `alloc`
