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
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking hashbrown v0.11.0
    Checking addr2line v0.16.0
error[E0599]: no method named `try_reserve` found for struct `Buf` in the current scope
    |
    |
303 |         self.inner.try_reserve(additional)
    |                    ^^^^^^^^^^^ method not found in `Buf`
   ::: library/std/src/sys/windows/os_str.rs:12:1
    |
12  | pub struct Buf {
12  | pub struct Buf {
    | -------------- method `try_reserve` not found for this

error[E0599]: no method named `try_reserve_exact` found for struct `Buf` in the current scope
    |
    |
371 |         self.inner.try_reserve_exact(additional)
    |                    ^^^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `reserve_exact`
   ::: library/std/src/sys/windows/os_str.rs:12:1
    |
12  | pub struct Buf {
12  | pub struct Buf {
    | -------------- method `try_reserve_exact` not found for this
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:18
