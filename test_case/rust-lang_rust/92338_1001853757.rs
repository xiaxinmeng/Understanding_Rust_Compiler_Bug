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
    Checking hashbrown v0.11.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
error[E0599]: no method named `try_reserve` found for struct `Wtf8Buf` in the current scope
   --> library/std/src/sys/windows/os_str.rs:109:20
    |
109 |         self.inner.try_reserve(additional)
    |                    ^^^^^^^^^^^ method not found in `Wtf8Buf`
    |
   ::: library/std/src/sys_common/wtf8.rs:117:1
    |
117 | pub struct Wtf8Buf {
    | ------------------ method `try_reserve` not found for this

error[E0599]: no method named `try_reserve_exact` found for struct `Wtf8Buf` in the current scope
   --> library/std/src/sys/windows/os_str.rs:117:20
    |
117 |         self.inner.try_reserve_exact(additional)
    |                    ^^^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `reserve_exact`
    |
   ::: library/std/src/sys_common/wtf8.rs:117:1
    |
117 | pub struct Wtf8Buf {
    | ------------------ method `try_reserve_exact` not found for this
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:18
