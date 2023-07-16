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
    Checking miniz_oxide v0.4.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0658]: `let` expressions in this position are unstable
    |
    |
116 |         if let Some(parser) = parser.strip_prefix(r"?\") && !parser.prefix_bytes().iter().any(|&x| x == '/') {
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable
error[E0308]: mismatched types
   --> library/std/src/sys/windows/path.rs:116:105
    |
    |
116 |         if let Some(parser) = parser.strip_prefix(r"?\") && !parser.prefix_bytes().iter().any(|&x| x == '/') {
    |                                                                                                         ^^^ expected `u8`, found `char`
Some errors have detailed explanations: E0308, E0658.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:15
