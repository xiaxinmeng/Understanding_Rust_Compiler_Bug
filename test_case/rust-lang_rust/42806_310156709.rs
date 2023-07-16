
Documenting stage2 compiler (x86_64-pc-windows-msvc)
 Documenting arena v0.0.0 (file:///E:/Source/rust/src/libarena)
 Documenting rustc-serialize v0.3.24
 Documenting fmt_macros v0.0.0 (file:///E:/Source/rust/src/libfmt_macros)
 Documenting rustc_platform_intrinsics v0.0.0 (file:///E:/Source/rust/src/librustc_platform_intrinsics)
 Documenting log v0.3.8
 Documenting bitflags v0.8.2
 Documenting serialize v0.0.0 (file:///E:/Source/rust/src/libserialize)
 Documenting gcc v0.3.51
error: use of unstable library feature 'rustc_private': mainly needed for compiler internals (see issue #27812)
   --> src\libfmt_macros\lib.rs:422:34
    |
422 |             Some(&(pos, c)) if c.is_xid_start() => {
    |                                  ^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable

error: use of unstable library feature 'rustc_private': mainly needed for compiler internals (see issue #27812)
   --> src\libfmt_macros\lib.rs:431:18
    |
431 |             if c.is_xid_continue() {
    |                  ^^^^^^^^^^^^^^^
    |
    = help: add #![feature(rustc_private)] to the crate attributes to enable

error: Compilation failed, aborting rustdoc

error: Could not document `fmt_macros`.
