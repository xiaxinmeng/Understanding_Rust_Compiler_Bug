plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.11.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.14.0
error[E0609]: no field `0` on type `os::imp::windows::io::handle::OwnedHandle`
   --> library/std/src/os/./windows/io/handle.rs:130:25
    |
130 |         if owned_handle.0.as_ptr() == c::INVALID_HANDLE_VALUE {
    |                         ^ unknown field
    = note: available fields are: `handle`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> library/std/src/os/./windows/io/handle.rs:133:13
    |
117 |     fn try_from(handle_or_invalid: HandleOrInvalid) -> Result<Self, ()> {
    |                                                        ---------------- expected `core::result::Result<os::imp::windows::io::handle::OwnedHandle, ()>` because of return type
133 |             owned_handle
    |             ^^^^^^^^^^^^
    |             |
    |             |
    |             expected enum `core::result::Result`, found struct `os::imp::windows::io::handle::OwnedHandle`
    |             help: try using a variant of the expected enum: `core::prelude::v1::Ok(owned_handle)`
    |
    = note: expected enum `core::result::Result<os::imp::windows::io::handle::OwnedHandle, ()>`
             found struct `os::imp::windows::io::handle::OwnedHandle`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0609.
For more information about an error, try `rustc --explain E0308`.
