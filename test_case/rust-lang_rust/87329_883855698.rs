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
    Checking miniz_oxide v0.4.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
error[E0308]: mismatched types
   --> library/std/src/os/./windows/io/handle.rs:105:28
105 |     pub const fn none() -> Self {
    |                  ----      ^^^^ expected struct `OptionFileHandle`, found `()`
    |                  |
    |                  |
    |                  implicitly returns `()` as its body has no tail or `return` expression
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `std`
