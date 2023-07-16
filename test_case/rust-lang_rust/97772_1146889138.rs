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
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
  --> src/librustdoc/docfs.rs:95:21
   |
95 | /                     sender.send(format!("\"{}\": {}", path.display(), e)).unwrap_or_else(|_| {
96 | |                         panic!("failed to send error on \"{}\"", path.display())
   | |______________________^ expected struct `std::fs::File`, found `()`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to previous error
