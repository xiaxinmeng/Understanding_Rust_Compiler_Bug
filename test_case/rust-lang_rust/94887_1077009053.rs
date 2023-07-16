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
    Checking addr2line v0.16.0
error[E0308]: mismatched types
  --> library/std/src/sys/windows/path.rs:71:30
   |
71 |         Self { path, prefix: Self::get_prefix(path, prefix_len) }
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected array `[u8; LEN]`, found struct `alloc_crate::vec::Vec`
   |
   = note: expected array `[u8; LEN]`
             found struct `alloc_crate::vec::Vec<u8>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:18
