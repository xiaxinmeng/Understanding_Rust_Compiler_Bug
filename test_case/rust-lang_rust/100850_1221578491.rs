plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.12.3
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0599]: no method named `sandbox` found for struct `sys::windows::process::Command` in the current scope
     |
1038 |         self.inner.sandbox()
     |                    ^^^^^^^ method not found in `sys::windows::process::Command`
     |
     |
    ::: library/std/src/sys/windows/process.rs:158:1
     |
158  | pub struct Command {
     | ------------------ method `sandbox` not found for this struct

error[E0599]: no method named `is_sandboxed` found for struct `sys::windows::process::Command` in the current scope
     |
     |
1046 |         self.inner.is_sandboxed()
     |                    ^^^^^^^^^^^^ method not found in `sys::windows::process::Command`
    ::: library/std/src/sys/windows/process.rs:158:1
     |
158  | pub struct Command {
158  | pub struct Command {
     | ------------------ method `is_sandboxed` not found for this struct
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:19
