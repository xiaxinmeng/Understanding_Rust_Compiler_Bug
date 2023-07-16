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
    Checking hashbrown v0.12.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
error[E0432]: unresolved import `crate::os::raw::NonZero_c_ulong`
 --> library/std/src/sys/windows/c.rs:8:5
  |
8 | use crate::os::raw::NonZero_c_ulong;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `NonZero_c_ulong` in `os::raw`
error: unused import: `TryFrom`
 --> library/std/src/sys/windows/process.rs:8:22
  |
  |
8 | use crate::convert::{TryFrom, TryInto};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:30
