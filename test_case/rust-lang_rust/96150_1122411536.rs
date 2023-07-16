plain
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
[RUSTC-TIMING] build_script_build test:false 0.260
[RUSTC-TIMING] build_script_build test:false 0.451
error: 2nd rule of macro `simd_shuffle16` is never used
    |
    |
154 |     ($x:expr, $y:expr, $idx:expr $(,)?) => {{
    |
    |
    = note: `-D unused-macro-rules` implied by `-D warnings`
[RUSTC-TIMING] core test:false 10.987
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:10:51
