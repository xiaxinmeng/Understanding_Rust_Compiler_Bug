
Building stage0 compiler artifacts (mips64el-unknown-linux-gnuabi64 -> mips64el-unknown-linux-gnuabi64)
   Compiling arena v0.0.0 (file:///opt/store/src/rust/src/libarena)
   Compiling log v0.0.0 (file:///opt/store/src/rust/src/liblog)
   Compiling rustc_bitflags v0.0.0 (file:///opt/store/src/rust/src/librustc_bitflags)
   Compiling graphviz v0.0.0 (file:///opt/store/src/rust/src/libgraphviz)
   Compiling rustdoc v0.0.0 (file:///opt/store/src/rust/src/librustdoc)
   Compiling rustc_llvm v0.0.0 (file:///opt/store/src/rust/src/librustc_llvm)
   Compiling flate v0.0.0 (file:///opt/store/src/rust/src/libflate)
   Compiling fmt_macros v0.0.0 (file:///opt/store/src/rust/src/libfmt_macros)
   Compiling serialize v0.0.0 (file:///opt/store/src/rust/src/libserialize)
   Compiling rustc_platform_intrinsics v0.0.0 (file:///opt/store/src/rust/src/librustc_platform_intrinsics)
   Compiling rustc_data_structures v0.0.0 (file:///opt/store/src/rust/src/librustc_data_structures)
   Compiling syntax_pos v0.0.0 (file:///opt/store/src/rust/src/libsyntax_pos)
   Compiling rustc_errors v0.0.0 (file:///opt/store/src/rust/src/librustc_errors)
   Compiling syntax v0.0.0 (file:///opt/store/src/rust/src/libsyntax)
   Compiling rustc_const_math v0.0.0 (file:///opt/store/src/rust/src/librustc_const_math)
   Compiling rustc_back v0.0.0 (file:///opt/store/src/rust/src/librustc_back)
   Compiling proc_macro_tokens v0.0.0 (file:///opt/store/src/rust/src/libproc_macro_tokens)
   Compiling proc_macro v0.0.0 (file:///opt/store/src/rust/src/libproc_macro)
   Compiling syntax_ext v0.0.0 (file:///opt/store/src/rust/src/libsyntax_ext)
   Compiling rustc v0.0.0 (file:///opt/store/src/rust/src/librustc)
*** Error in `/opt/store/src/rust/build/mips64el-unknown-linux-gnuabi64/stage0/bin/rustc': double free or corruption (out): 0x000000ffe155dea0 ***
error: Could not compile `rustc`.

To learn more, run the command again with --verbose.


command did not execute successfully: "/opt/store/src/rust/build/mips64el-unknown-linux-gnuabi64/stage0/bin/cargo" "build" "-j" "4" "--target" "mips64el-unknown-linux-gnuabi64" "--release" "-
-features" " jemalloc" "--manifest-path" "/opt/store/src/rust/src/rustc/Cargo.toml"
expected success, got: exit code: 101
