
$ ./x.py build --keep-stage 1 --stage 2
Updating only changed submodules
Submodules updated in 0.09 seconds
    Finished dev [unoptimized] target(s) in 3.52s
Building stage0 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.29s
Copying stage0 std from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 test artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.14s
Copying stage0 test from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 compiler artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.45s
Copying stage0 rustc from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 codegen artifacts (x86_64-apple-darwin -> x86_64-apple-darwin, llvm)
    Finished release [optimized] target(s) in 0.20s
Assembling stage1 compiler (x86_64-apple-darwin)
Warning: Using a potentially old libstd. This may not behave well.
Copying stage1 std from stage1 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Warning: Using a potentially old libtest. This may not behave well.
Copying stage1 test from stage1 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
thread 'main' panicked at 'fs::read(stamp) failed with No such file or directory (os error 2)', src/bootstrap/lib.rs:1159:24
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
failed to run: /Users/rjacobson/rust/build/bootstrap/debug/bootstrap build --keep-stage 1 --stage 2
Build completed unsuccessfully in 0:00:09
