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
  --> library/std/src/sys/windows/io.rs:87:32
   |
87 |     unsafe { handle_is_console(h.as_handle()) }
   |              |                 |
   |              |                 |
   |              |                 expected `&os::windows::io::handle::BorrowedHandle<'_>`, found struct `os::windows::io::handle::BorrowedHandle`
   |              |                 help: consider borrowing here: `&h.as_handle()`
   |
note: function defined here
  --> library/std/src/sys/windows/io.rs:90:11
   |
   |
90 | unsafe fn handle_is_console(handle: &BorrowedHandle<'_>) -> bool {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:16
