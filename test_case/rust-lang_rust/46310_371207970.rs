rust
#![feature(repr_simd, simd_ffi, link_llvm_intrinsics, target_feature)]

#[allow(non_camel_case_types)]
#[repr(simd)]
pub struct i8x16(
    i8, i8, i8, i8, i8, i8, i8, i8,
    i8, i8, i8, i8, i8, i8, i8, i8
);

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.mips.add.a.b"]
    fn msa_add_a_b(a: i8x16, b: i8x16) -> i8x16;
}

#[target_feature(enable = "msa")]
pub unsafe fn foo(a: i8x16) -> i8x16 {
    let b = i8x16(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    msa_add_a_b(a, b)
}
