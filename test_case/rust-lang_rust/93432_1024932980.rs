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
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error: literal out of range for `usize`
   --> compiler/rustc_data_structures/src/stable_hasher/tests.rs:161:22
    |
161 |     check_hash(0xFF, 0xFFFFFFFFFFFFFFFF);
    |
    = note: `#[deny(overflowing_literals)]` on by default
    = note: `#[deny(overflowing_literals)]` on by default
    = note: the literal `0xFFFFFFFFFFFFFFFF` (decimal `18446744073709551615`) does not fit into the type `usize` and will become `4294967295usize`
error: could not compile `rustc_data_structures` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:00:40
