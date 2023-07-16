plain
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
[RUSTC-TIMING] build_script_build test:false 0.312
[RUSTC-TIMING] build_script_build test:false 0.511
error: 1st rule of macro `simd_shuffle2` is never used
   |
   |
98 |     ($x:expr, $y:expr, <$(const $imm:ident : $ty:ty),+ $(,)?> $idx:expr $(,)?) => {{
   |
   |
   = note: `-D unused-macro-rules` implied by `-D warnings`

error: 3rd rule of macro `impl_vec_trait` is never used
   --> library/core/src/../../stdarch/crates/core_arch/src/powerpc/altivec.rs:379:9
    |
379 |         ([$Trait:ident $m:ident] 1 ($ub:ident, $sb:ident, $uh:ident, $sh:ident, $uw:ident, $sw:ident, $sf: ident)) => {


error: 9th rule of macro `impl_vec_trait` is never used
   --> library/core/src/../../stdarch/crates/core_arch/src/powerpc/altivec.rs:422:9
    |
422 |         ([$Trait:ident $m:ident] 2 ($fn:ident)) => {

[RUSTC-TIMING] core test:false 11.298
error: could not compile `core` due to 3 previous errors
Build completed unsuccessfully in 0:05:07
