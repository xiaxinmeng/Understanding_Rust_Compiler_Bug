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
   --> library/std/src/sys/windows/args.rs:318:65
    |
318 |     if script.contains(&(b'"' as u16)) || script.last() == Some(&b'\\' as u16) {
    |                                                                 |
    |                                                                 |
    |                                                                 expected `&u16`, found `u16`
    |                                                                 help: consider borrowing here: `&(&b'\\' as u16)`

error[E0606]: casting `&u8` as `u16` is invalid
   --> library/std/src/sys/windows/args.rs:318:65
    |
318 |     if script.contains(&(b'"' as u16)) || script.last() == Some(&b'\\' as u16) {
    |                                                                 |
    |                                                                 |
    |                                                                 cannot cast `&u8` as `u16`
    |                                                                 help: dereference the expression: `*&b'\\'`
Some errors have detailed explanations: E0308, E0606.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:15
