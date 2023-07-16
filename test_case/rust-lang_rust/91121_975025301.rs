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
  --> library/std/src/sys/windows/io.rs:84:25
   |
84 | impl io::IsTerminal for sys::stdio::Stdin {
   |                         ^^^ use of undeclared crate or module `sys`
error[E0433]: failed to resolve: use of undeclared crate or module `sys`
   --> library/std/src/sys/windows/io.rs:110:25
    |
    |
110 | impl io::IsTerminal for sys::stdio::Stdout {
    |                         ^^^ use of undeclared crate or module `sys`
error[E0433]: failed to resolve: use of undeclared crate or module `sys`
   --> library/std/src/sys/windows/io.rs:136:25
    |
    |
136 | impl io::IsTerminal for sys::stdio::Stderr {
    |                         ^^^ use of undeclared crate or module `sys`
error[E0433]: failed to resolve: use of undeclared crate or module `std`
   --> library/std/src/sys/windows/io.rs:174:16
    |
    |
174 |     let size = std::mem::size_of::<c::FILE_NAME_INFO>();
    |                ^^^ use of undeclared crate or module `std`
error[E0433]: failed to resolve: use of undeclared crate or module `std`
   --> library/std/src/sys/windows/io.rs:175:62
    |
    |
175 |     let mut name_info_bytes = vec![0u8; size + c::MAX_PATH * std::mem::size_of::<WCHAR>()];
    |                                                              ^^^ use of undeclared crate or module `std`
error[E0433]: failed to resolve: use of undeclared crate or module `std`
   --> library/std/src/sys/windows/io.rs:186:13
    |
186 |     let s = std::slice::from_raw_parts(
186 |     let s = std::slice::from_raw_parts(
    |             ^^^ use of undeclared crate or module `std`

error[E0425]: cannot find value `MAX_PATH` in module `c`
   --> library/std/src/sys/windows/io.rs:175:51
    |
175 |     let mut name_info_bytes = vec![0u8; size + c::MAX_PATH * std::mem::size_of::<WCHAR>()];
    |                                                   ^^^^^^^^ not found in `c`
    |
note: constant `crate::backtrace_rs::windows::MAX_PATH` exists but is inaccessible
    |
51  | / macro_rules! ffi {
52  | |     () => ();
53  | |
53  | |
54  | |     (#[repr($($r:tt)*)] pub struct $name:ident { $(pub $field:ident: $ty:ty,)* } $($rest:tt)*) => (
...   |
100 | |         ffi!($($rest)*);
    | |         |
    | |         in this macro invocation (#2)
    | |         in this macro invocation (#4)
    | |         in this macro invocation (#7)
    | |         in this macro invocation (#7)
    | |         in this macro invocation (#8)
...   |
144 | |         ffi!($($rest)*);
    | |         |
    | |         in this macro invocation (#3)
    | |         in this macro invocation (#5)
    | |         in this macro invocation (#6)
    | |         in this macro invocation (#6)
...   |
148 | |         pub const $name: $ty = $val;
    | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible
...   |
162 | |         ffi!($($rest)*);
    | |         |
    | |         in this macro invocation (#9)
    | |         in this macro invocation (#10)
...   |
...   |
197 | |     );
198 | | }
    | | -
    | | |
    | | in this expansion of `ffi!` (#1)
    | | in this expansion of `ffi!` (#2)
    | | in this expansion of `ffi!` (#3)
    | | in this expansion of `ffi!` (#4)
    | | in this expansion of `ffi!` (#5)
    | | in this expansion of `ffi!` (#6)
    | | in this expansion of `ffi!` (#7)
    | | in this expansion of `ffi!` (#8)
    | |_in this expansion of `ffi!` (#9)
    |   in this expansion of `ffi!` (#10)
199 | 
200 | / ffi! {
201 | |     #[repr(C)]
202 | |     pub struct STACKFRAME64 {
203 | |         pub AddrPC: ADDRESS64,
436 | |     }
437 | | }
    | |_- in this macro invocation (#1)


error[E0412]: cannot find type `WCHAR` in this scope
   --> library/std/src/sys/windows/io.rs:175:82
    |
175 |     let mut name_info_bytes = vec![0u8; size + c::MAX_PATH * std::mem::size_of::<WCHAR>()];
    |
help: consider importing this type alias
    |
    |
1   | use crate::sys::c::WCHAR;


error[E0425]: cannot find function, tuple struct or tuple variant `GetFileInformationByHandleEx` in module `c`
   --> library/std/src/sys/windows/io.rs:176:18
    |
176 |       let res = c::GetFileInformationByHandleEx(
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `GetFileInformationByHandle`
    |
   ::: library/std/src/sys/windows/c.rs:728:9
728 | /         pub fn GetFileInformationByHandle(
728 | /         pub fn GetFileInformationByHandle(
729 | |             hFile: HANDLE,
730 | |             lpFileInformation: LPBY_HANDLE_FILE_INFORMATION,
731 | |         ) -> BOOL;
    | |__________________- similarly named function `GetFileInformationByHandle` defined here
error[E0603]: enum import `c_void` is private
   --> library/std/src/sys/windows/io.rs:179:52
    |
    |
179 |         &mut *name_info_bytes as *mut _ as *mut c::c_void,
    |                                                    ^^^^^^ private enum import
note: the enum import `c_void` is defined here...
   --> library/std/src/sys/windows/c.rs:11:12
    |
    |
11  | use libc::{c_void, size_t, wchar_t};
    |            ^^^^^^
note: ...and refers to the enum `c_void` which is defined here
    |
    |
101 |         pub use windows::*;
    |                 ^^^^^^^^^^ consider importing it directly

error[E0599]: no function or associated item named `is_terminal` found for struct `sys::windows::stdio::Stdin` in the current scope
     |
1217 |                stdio::Stdin::is_terminal()
     |                              ^^^^^^^^^^^ function or associated item not found in `sys::windows::stdio::Stdin`
     |
     |
    ::: library/std/src/sys/windows/stdio.rs:16:1
     |
16   | pub struct Stdin {
     | ---------------- function or associated item `is_terminal` not found for this

error[E0599]: no function or associated item named `is_terminal` found for struct `sys::windows::stdio::Stdout` in the current scope
     |
1224 |                 stdio::Stdout::is_terminal()
     |                                ^^^^^^^^^^^ function or associated item not found in `sys::windows::stdio::Stdout`
     |
     |
    ::: library/std/src/sys/windows/stdio.rs:19:1
     |
19   | pub struct Stdout {
     | ----------------- function or associated item `is_terminal` not found for this

error[E0599]: no function or associated item named `is_terminal` found for struct `sys::windows::stdio::Stderr` in the current scope
     |
1231 |                 stdio::Stderr::is_terminal()
     |                                ^^^^^^^^^^^ function or associated item not found in `sys::windows::stdio::Stderr`
     |
     |
    ::: library/std/src/sys/windows/stdio.rs:23:1
     |
23   | pub struct Stderr {
     | ----------------- function or associated item `is_terminal` not found for this
error[E0616]: field `FileName` of struct `FILE_NAME_INFO` is private
   --> library/std/src/sys/windows/io.rs:187:19
    |
187 |         name_info.FileName.as_ptr(),
187 |         name_info.FileName.as_ptr(),
    |                   ^^^^^^^^ private field

error[E0616]: field `FileNameLength` of struct `FILE_NAME_INFO` is private
   --> library/std/src/sys/windows/io.rs:188:19
188 |         name_info.FileNameLength as usize / 2,
    |                   ^^^^^^^^^^^^^^ private field

Some errors have detailed explanations: E0412, E0425, E0433, E0599, E0603, E0616.
