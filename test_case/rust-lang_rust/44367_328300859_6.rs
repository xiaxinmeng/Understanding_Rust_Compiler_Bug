rust
#[target_feature = "+avx2"] unsafe fn foo(a: u8x32<AVX2_ABI>) {}
#[target_feature = "+avx2"]
unsafe fn bar() {
    FOO = foo; // OK or Error?
}
