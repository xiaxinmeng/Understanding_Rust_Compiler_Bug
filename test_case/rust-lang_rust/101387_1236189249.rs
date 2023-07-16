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
    Checking addr2line v0.16.0
error[E0599]: no method named `interrupt` found for struct `Process` in the current scope
    --> library/std/src/process.rs:1891:21
     |
1891 |         self.handle.interrupt()
     |                     ^^^^^^^^^ method not found in `Process`
    ::: library/std/src/sys/windows/process.rs:584:1
     |
584  | pub struct Process {
584  | pub struct Process {
     | ------------------ method `interrupt` not found for this struct
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:21
