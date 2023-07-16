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
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0412]: cannot find type `BorrowedHandle` in this scope
  --> library/std/src/sys/windows/process/tests.rs:38:28
   |
38 |         fn ResumeThread(_: BorrowedHandle<'_>) -> u32;
   |
help: consider importing this struct
   |
1  | use crate::os::windows::prelude::BorrowedHandle;
---
   |
29 |     use crate::os::windows::io::AsRawHandle;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0412`.
error: could not compile `std` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:31
