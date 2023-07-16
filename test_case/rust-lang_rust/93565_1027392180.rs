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
    Checking hashbrown v0.12.0
    Checking miniz_oxide v0.4.0
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0599]: no method named `try_branch` found for struct `sys::windows::process::ExitStatus` in the current scope
     |
     |
1531 |         self.0.try_branch()
     |                ^^^^^^^^^^ method not found in `sys::windows::process::ExitStatus`
    ::: library/std/src/sys/windows/process.rs:605:1
     |
     |
605  | pub struct ExitStatus(c::DWORD);
     | -------------------------------- method `try_branch` not found for this

error[E0599]: no function or associated item named `zero_status` found for struct `sys::windows::process::ExitStatus` in the current scope
     |
     |
1535 |         Self(imp::ExitStatus::zero_status())
     |                               ^^^^^^^^^^^ function or associated item not found in `sys::windows::process::ExitStatus`
    ::: library/std/src/sys/windows/process.rs:605:1
     |
     |
605  | pub struct ExitStatus(c::DWORD);
     | -------------------------------- function or associated item `zero_status` not found for this
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:15
