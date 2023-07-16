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
    | |_______________^
    | 
   ::: library/core/src/num/mod.rs:98:5
    |
98  | /     int_impl! { i8, i8, u8, 8, 7, -128, 127, 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
99  | |     "[0x12]", "[0x12]", "", "" }
    |
    |
    = note: `-D rustdoc::invalid-rust-codeblocks` implied by `-D warnings`
    = note: this error originates in the macro `int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
    |
928 |         /// 