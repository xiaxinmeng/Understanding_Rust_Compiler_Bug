
49.31sBuilding stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:33:34]    Compiling cc v1.0.25
[00:33:34]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:33:34]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:33:34]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:33:41]    Compiling compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[00:33:41]    Compiling cmake v0.1.33
[00:33:41]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:33:45]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:33:45]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:33:45]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:33:46]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:34:19] rustc: /checkout/src/llvm/include/llvm/Support/Casting.h:106: static bool llvm::isa_impl_cl<To, const From*>::doit(const From*) [with To = llvm::MDTuple; From = llvm::Metadata]: Assertion `Val && "isa<> used on a null pointer"' failed.
[00:34:23] [RUSTC-TIMING] core test:false 48.883
[00:34:23] rustc exited with signal: 6
[00:34:23] error: Could not compile `core`.
[00:34:23] 
[00:34:23] To learn more, run the command again with --verbose.
[00:34:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:34:23] expected success, got: exit code: 101
[00:34:23] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:34:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
