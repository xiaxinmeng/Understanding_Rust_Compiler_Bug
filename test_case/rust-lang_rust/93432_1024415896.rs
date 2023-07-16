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
    Checking rustc_log v0.0.0 (/checkout/compiler/rustc_log)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error: literal out of range for `isize`
   --> compiler/rustc_data_structures/src/stable_hasher/tests.rs:161:26
    |
161 |     check_hash(0xAAAAAA, 0xAAAAAAAA);
    |
    = note: `#[deny(overflowing_literals)]` on by default
    = note: `#[deny(overflowing_literals)]` on by default
    = note: the literal `0xAAAAAAAA` (decimal `2863311530`) does not fit into the type `isize` and will become `-1431655766isize`
error: could not compile `rustc_data_structures` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:00:43
