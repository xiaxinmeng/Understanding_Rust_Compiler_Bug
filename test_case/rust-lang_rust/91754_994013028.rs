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
error: casts cannot be followed by ?
   --> library/std/src/sys/windows/stdio.rs:257:34
    |
257 |             let read_bytes: u8 = utf16_to_utf8(&utf16_buf[..read], &mut self.incomplete_utf8.bytes)? as u8?;
    |
help: try surrounding the expression in parentheses
    |
    |
257 |             let read_bytes: u8 = (utf16_to_utf8(&utf16_buf[..read], &mut self.incomplete_utf8.bytes)? as u8)?;
    |                                  +                                                                         +
error[E0277]: the `?` operator can only be applied to values that implement `Try`
   --> library/std/src/sys/windows/stdio.rs:257:34
    |
    |
257 |             let read_bytes: u8 = utf16_to_utf8(&utf16_buf[..read], &mut self.incomplete_utf8.bytes)? as u8?;
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `u8`
    = help: the trait `Try` is not implemented for `u8`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `std` due to 2 previous errors
