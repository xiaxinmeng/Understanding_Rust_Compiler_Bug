plain
[00:05:05]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:05:06]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:05:07]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:05:08]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:05:12] warning: type `c_void` should have a camel case name such as `CVoid`
[00:05:12]   --> libcore/ffi.rs:26:1
[00:05:12]    |
[00:05:12] 26 | / pub enum c_void {
[00:05:12] 27 | |     #[unstable(feature = "c_void_variant", reason = "should not have to exist",
[00:05:12] 28 | |                issue = "0")]
[00:05:12] 29 | |     #[doc(hidden)] __variant1,
[00:05:12] ...  |
[00:05:12] 32 | |     #[doc(hidden)] __variant2,
[00:05:12]    | |_^
[00:05:12]    |
[00:05:12]    = note: #[warn(non_camel_case_types)] on by default
[00:05:12] 
[00:05:12] 
[00:05:12] warning: variant `__variant1` should have a camel case name such as `Variant1`
[00:05:12]   --> libcore/ffi.rs:29:20
[00:05:12]    |
[00:05:12] 29 |     #[doc(hidden)] __variant1,
[00:05:12] 
[00:05:12] 
[00:05:12] warning: variant `__variant2` should have a camel case name such as `Variant2`
[00:05:12]   --> libcore/ffi.rs:32:20
[00:05:12]    |
[00:05:12] 32 |     #[doc(hidden)] __variant2,
[00:05:12] 
[00:05:19]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:05:19]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:05:20]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
