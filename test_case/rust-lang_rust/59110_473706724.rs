
Updating only changed submodules
Submodules updated in 0.11 seconds
    Finished dev [unoptimized] target(s) in 2.28s
Building stage0 std artifacts (x86_64-apple-darwin -> armv7-apple-ios)
   Compiling core v0.0.0 (/Users/forcebru/Desktop/Test/RUST/rust/src/libcore)
   Compiling libc v0.2.46
   Compiling compiler_builtins v0.1.5
   Compiling unwind v0.0.0 (/Users/forcebru/Desktop/Test/RUST/rust/src/libunwind)
   Compiling backtrace-sys v0.1.27
   Compiling std v0.0.0 (/Users/forcebru/Desktop/Test/RUST/rust/src/libstd)
   Compiling rustc-std-workspace-core v1.0.0 (/Users/forcebru/Desktop/Test/RUST/rust/src/tools/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/Users/forcebru/Desktop/Test/RUST/rust/src/liballoc)
   Compiling panic_abort v0.0.0 (/Users/forcebru/Desktop/Test/RUST/rust/src/libpanic_abort)
   Compiling rustc-demangle v0.1.10
   Compiling panic_unwind v0.0.0 (/Users/forcebru/Desktop/Test/RUST/rust/src/libpanic_unwind)
warning: dropping unsupported crate type `dylib` for target `armv7-apple-ios`2: std                                                                                                                                                       

    Finished release [optimized] target(s) in 1m 00s
Copying stage0 std from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / armv7-apple-ios)
Building stage0 test artifacts (x86_64-apple-darwin -> armv7-apple-ios)
   Compiling proc_macro v0.0.0 (/Users/forcebru/Desktop/Test/RUST/rust/src/libproc_macro)
   Compiling getopts v0.2.17
   Compiling term v0.0.0 (/Users/forcebru/Desktop/Test/RUST/rust/src/libterm)
warning: dropping unsupported crate type `dylib` for target `armv7-apple-ios`4: proc_macro, getopts, term                                                                                                                                 

   Compiling test v0.0.0 (/Users/forcebru/Desktop/Test/RUST/rust/src/libtest)
warning: dropping unsupported crate type `dylib` for target `armv7-apple-ios`4: test                                                                                                                                                      

    Finished release [optimized] target(s) in 17.29s
Copying stage0 test from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / armv7-apple-ios)
Building stage0 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 1.28s
Copying stage0 std from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 test artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.50s
Copying stage0 test from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 compiler artifacts (x86_64-apple-darwin -> armv7-apple-ios)
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/Users/forcebru/Desktop/Test/RUST/rust/build_macos/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names --target armv7-apple-ios --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro` (exit code: 1)
--- stderr
error: unknown debugging option: `dual-proc-macros`


command did not execute successfully: "/Users/forcebru/Desktop/Test/RUST/rust/build_macos/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "armv7-apple-ios" "-Zdual-proc-macros" "-j" "4" "--release" "--features" "" "--manifest-path" "/Users/forcebru/Desktop/Test/RUST/rust/src/rustc/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
failed to run: /Users/forcebru/Desktop/Test/RUST/rust/build_macos/build/bootstrap/debug/bootstrap build -i --stage 1 -j 4 --target armv7-apple-ios --host armv7-apple-ios src/libstd
Build completed unsuccessfully in 0:01:25
