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
    Checking hashbrown v0.12.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error: associated function is never used: `from_raw_handle`
   --> library/std/src/os/windows/io/handle.rs:235:15
    |
235 |     unsafe fn from_raw_handle(handle: RawHandle) -> Self {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: associated function is never used: `from_raw_handle`
error: associated function is never used: `from_raw_handle`
   --> library/std/src/os/windows/io/handle.rs:257:15
    |
257 |     unsafe fn from_raw_handle(handle: RawHandle) -> Self {

error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:16
