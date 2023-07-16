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
error[E0432]: unresolved import `super::thread_local_dtor`
 --> library/std/src/sys/windows/thread_local_key.rs:1:12
  |
1 | use super::thread_local_dtor::run_keyless_dtors;
  |            ^^^^^^^^^^^^^^^^^ could not find `thread_local_dtor` in `super`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:17
