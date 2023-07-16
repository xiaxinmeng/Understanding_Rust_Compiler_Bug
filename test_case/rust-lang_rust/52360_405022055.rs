
Updating only changed submodules
Submodules updated in 0.03 seconds
   Compiling bootstrap v0.0.0 (file:///home/r/src/rust/rustc/src/bootstrap)
    Finished dev [unoptimized] target(s) in 5.67s
Warning: Using a potentially old librustc. This may not behave well.
Warning: Using a potentially old codegen backend. This may not behave well.
Warning: Using a potentially old librustc. This may not behave well.
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.24s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.22s
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_mir v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_mir)
   Compiling rustc_passes v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_passes)
   Compiling rustc_borrowck v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_borrowck)
   Compiling rustc_codegen_utils v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_codegen_utils)
   Compiling rustc_lint v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_lint)
   Compiling rustc_driver v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_driver)
   Compiling rustc-main v0.0.0 (file:///home/r/src/rust/rustc/src/rustc)
    Finished release [optimized] target(s) in 14.51s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
   Compiling rustc_codegen_llvm v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_codegen_llvm)
    Finished release [optimized] target(s) in 7.64s
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Warning: Using a potentially old librustc. This may not behave well.
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Warning: Using a potentially old codegen backend. This may not behave well.
Assembling stage2 compiler (x86_64-unknown-linux-gnu)
Warning: Using a potentially old librustc. This may not behave well.
Copying stage2 rustc from stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
thread 'main' panicked at 'File::open(stamp) failed with No such file or directory (os error 2)', bootstrap/lib.rs:1109:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: /home/r/src/rust/rustc/build/bootstrap/debug/bootstrap build src/rustc --keep-stage 1
Build completed unsuccessfully in 0:00:29
