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
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.12.3
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error[E0603]: module `ranged` is private
  --> library/std/src/os/windows/io/socket.rs:16:16
16 | use core::num::ranged::Ranged;
   |                ^^^^^^ private module
   |
note: the module `ranged` is defined here
note: the module `ranged` is defined here
  --> /checkout/library/core/src/num/mod.rs:48:1
   |
48 | mod ranged;

For more information about this error, try `rustc --explain E0603`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:23
