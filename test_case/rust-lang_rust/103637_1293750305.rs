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
    Checking hashbrown v0.12.3
    Checking miniz_oxide v0.4.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
error[E0428]: the name `LPCVOID` is defined multiple times
   --> library/std/src/sys/windows/c.rs:830:5
    |
59  | pub type LPCVOID = *const c_void;
    | --------------------------------- previous definition of the type `LPCVOID` here
...
830 |     pub type LPCVOID = *const c_void;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `LPCVOID` redefined here
    |
    = note: `LPCVOID` must be defined only once in the type namespace of this module
For more information about this error, try `rustc --explain E0428`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:28
