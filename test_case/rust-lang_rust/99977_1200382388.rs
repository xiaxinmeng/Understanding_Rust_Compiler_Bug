plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: any use of this value will cause an error
   --> /checkout/library/core/src/ffi/c_str.rs:406:28
    |
406 |                 let byte = bytes[i];
    |                            |
    |                            index out of bounds: the length is 1 but the index is usize::MAX
    |                            inside `CStr::from_bytes_with_nul_unchecked::const_impl` at /checkout/library/core/src/ffi/c_str.rs:406:28
    |                            inside `CStr::from_bytes_with_nul_unchecked::const_impl` at /checkout/library/core/src/ffi/c_str.rs:406:28
    |                            inside `<for<'r> fn(&'r [u8]) -> &'r CStr {CStr::from_bytes_with_nul_unchecked::const_impl} as std::ops::FnOnce<(&[u8],)>>::call_once - shim(for<'r> fn(&'r [u8]) -> &'r CStr {CStr::from_bytes_with_nul_unchecked::const_impl})` at /checkout/library/core/src/ops/function.rs:248:5
    |                            inside `std::intrinsics::const_eval_select::<(&[u8],), for<'r> fn(&'r [u8]) -> &'r CStr {CStr::from_bytes_with_nul_unchecked::const_impl}, for<'r> fn(&'r [u8]) -> &'r CStr {CStr::from_bytes_with_nul_unchecked::rt_impl}, &CStr>` at /checkout/library/core/src/intrinsics.rs:2710:5
    |                            inside `CStr::from_bytes_with_nul_unchecked` at /checkout/library/core/src/ffi/c_str.rs:418:18
    |                            inside `EMPTY_C_STR` at compiler/rustc_codegen_llvm/src/builder.rs:49:37
   ::: compiler/rustc_codegen_llvm/src/builder.rs:49:1
    |
    |
49  | const EMPTY_C_STR: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };
    |
    = note: `#[deny(const_err)]` on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> compiler/rustc_codegen_llvm/src/builder.rs:54:32
   |
54 | const UNNAMED: *const c_char = EMPTY_C_STR.as_ptr();
   |                                |
   |                                referenced constant has errors
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
