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
    Checking addr2line v0.16.0
error[E0308]: mismatched types
   --> library/std/src/sys/windows/io.rs:138:41
    |
138 |     let name = String::from_utf16_lossy(s);
    |                ------------------------ ^ expected `u16`, found `u8`
    |                arguments to this function are incorrect
    |
    = note: expected reference `&[u16]`
               found reference `&[u8]`
               found reference `&[u8]`
note: associated function defined here
   --> /checkout/library/alloc/src/string.rs:725:12
    |
725 |     pub fn from_utf16_lossy(v: &[u16]) -> String {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:23
