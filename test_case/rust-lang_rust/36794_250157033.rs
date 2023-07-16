
Testing librustc stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling fmt_macros v0.0.0 (file:///build/src/libfmt_macros)
   Compiling syntax_pos v0.0.0 (file:///build/src/libsyntax_pos)
   Compiling rustc_errors v0.0.0 (file:///build/src/librustc_errors)
   Compiling syntax v0.0.0 (file:///build/src/libsyntax)
   Compiling build_helper v0.1.0 (file:///build/src/build_helper)
   Compiling serialize v0.0.0 (file:///build/src/libserialize)
   Compiling flate v0.0.0 (file:///build/src/libflate)
   Compiling rustc_llvm v0.0.0 (file:///build/src/librustc_llvm)
   Compiling rustc_const_math v0.0.0 (file:///build/src/librustc_const_math)
   Compiling rustc_macro v0.0.0 (file:///build/src/librustc_macro)
   Compiling rustc_data_structures v0.0.0 (file:///build/src/librustc_data_structures)
   Compiling rustc_platform_intrinsics v0.0.0 (file:///build/src/librustc_platform_intrinsics)
   Compiling rustc_bitflags v0.0.0 (file:///build/src/librustc_bitflags)
   Compiling syntax_ext v0.0.0 (file:///build/src/libsyntax_ext)
   Compiling rustc_back v0.0.0 (file:///build/src/librustc_back)
   Compiling arena v0.0.0 (file:///build/src/libarena)
   Compiling log v0.0.0 (file:///build/src/liblog)
   Compiling rustc v0.0.0 (file:///build/src/librustc)
   Compiling rustc_save_analysis v0.0.0 (file:///build/src/librustc_save_analysis)
error[E0308]: mismatched types
    --> src/librustc/session/config.rs:2311:25
     |
2311 |         opts.cg.panic = PanicStrategy::Abort;
     |                         ^^^^^^^^^^^^^^^^^^^^ expected enum `core::option::Option`, found enum `rustc_back::PanicStrategy`
     |
     = note: expected type `core::option::Option<rustc_back::PanicStrategy>`
     = note:    found type `rustc_back::PanicStrategy`

error: aborting due to previous error
