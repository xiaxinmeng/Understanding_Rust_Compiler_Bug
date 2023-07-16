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
error[E0423]: expected value, found builtin type `u8`
   --> library/std/src/sys/windows/process.rs:671:33
    |
671 |         ExitCode(c::DWORD::from(u8))

For more information about this error, try `rustc --explain E0423`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:15
