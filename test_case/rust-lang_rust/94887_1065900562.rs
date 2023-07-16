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
    Checking miniz_oxide v0.4.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking addr2line v0.16.0
error: unnecessary `unsafe` block
  --> library/std/src/sys/windows/path/absolute.rs:24:18
13 | / pub(crate) unsafe fn get_full_path_name(
14 | |     lpfilename: *const u16,
15 | |     buffer: *mut u16,
15 | |     buffer: *mut u16,
16 | |     size: c::DWORD,
17 | | ) -> c::DWORD {
   | |_____________- because it's nested under this `unsafe` fn
...
24 |       let result = unsafe { c::GetFullPathNameW(lpfilename, size, buffer, ptr::null_mut()) };
   |                    ^^^^^^ unnecessary `unsafe` block
   |
   = note: `-D unused-unsafe` implied by `-D warnings`
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:16
