plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unused doc comment
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs:2901:1
     |
2901 |   /// LLVM intrinsics used in the above functions
     |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2902 |   #[allow(improper_ctypes)]
2903 | / extern "C" {
2904 | |     #[link_name = "llvm.x86.avx.addsub.pd.256"]
2905 | |     fn addsubpd256(a: __m256d, b: __m256d) -> __m256d;
2906 | |     #[link_name = "llvm.x86.avx.addsub.ps.256"]
...    |
3043 | |     fn vmaxpd(a: __m256d, b: __m256d) -> __m256d;
3044 | | }
     | |_- rustdoc does not generate documentation for extern block
     |
     = note: `-D unused-doc-comments` implied by `-D warnings`
     = help: use `//` for a plain comment
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:04:07
