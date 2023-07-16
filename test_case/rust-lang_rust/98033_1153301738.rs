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
error[E0433]: failed to resolve: use of undeclared crate or module `sys`
    --> library/std/src/io/stdio.rs:1058:17
     |
1058 |                 sys::io::handle_is_console(self.as_handle())
     |                 ^^^ use of undeclared crate or module `sys`

error[E0412]: cannot find type `BorrowedHandle` in this scope
  --> library/std/src/sys/windows/io.rs:88:41
   |
88 | pub unsafe fn handle_is_console(handle: BorrowedHandle<'_>) -> io::Result<bool> {
   |
help: consider importing this struct
   |
1  | use crate::os::windows::prelude::BorrowedHandle;
---
  |
6 | use crate::sys;
  |     ^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `crate::os::windows::io::AsRawHandle`
 --> library/std/src/sys/windows/io.rs:4:5
  |
4 | use crate::os::windows::io::AsRawHandle;
