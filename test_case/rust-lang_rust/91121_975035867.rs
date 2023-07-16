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
  |
7 | use std;
  |     ^^^ no external crate `std`

error[E0425]: cannot find function, tuple struct or tuple variant `GetFileInformationByHandleEx` in module `c`
   --> library/std/src/sys/windows/io.rs:179:18
    |
179 |       let res = c::GetFileInformationByHandleEx(
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `GetFileInformationByHandle`
    |
   ::: library/std/src/sys/windows/c.rs:730:9
730 | /         pub fn GetFileInformationByHandle(
730 | /         pub fn GetFileInformationByHandle(
731 | |             hFile: HANDLE,
732 | |             lpFileInformation: LPBY_HANDLE_FILE_INFORMATION,
733 | |         ) -> BOOL;
    | |__________________- similarly named function `GetFileInformationByHandle` defined here
Some errors have detailed explanations: E0425, E0432.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:17
