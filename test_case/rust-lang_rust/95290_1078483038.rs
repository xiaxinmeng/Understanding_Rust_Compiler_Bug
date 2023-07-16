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
    Checking object v0.26.2
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.12.0
    Checking addr2line v0.16.0
error[E0624]: associated function `as_u8_slice` is private
    |
715 |         self.inner.as_u8_slice()
    |                    ^^^^^^^^^^^ private associated function
    |
    |
   ::: library/std/src/sys/windows/os_str.rs:202:5
    |
202 |     fn as_u8_slice(&self) -> &[u8] {

For more information about this error, try `rustc --explain E0624`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:16
