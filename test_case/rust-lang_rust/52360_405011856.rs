
➜  rust git:(pull_52360) ./x.py test --keep-stage 1 --stage 2 src/test/pretty ; say "I am done. I am really done. ... Really"
Updating only changed submodules
Submodules updated in 0.09 seconds
    Finished dev [unoptimized] target(s) in 0.27s
Warning: Using a potentially old librustc. This may not behave well.
Warning: Using a potentially old codegen backend. This may not behave well.
Warning: Using a potentially old librustc. This may not behave well.
Warning: Using a potentially old libtest. This may not behave well.
Building stage0 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.28s
Copying stage0 std from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 test artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.26s
Copying stage0 test from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 compiler artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.34s
Copying stage0 rustc from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 codegen artifacts (x86_64-apple-darwin -> x86_64-apple-darwin, llvm)
    Finished release [optimized] target(s) in 0.34s
Assembling stage1 compiler (x86_64-apple-darwin)
Warning: Using a potentially old librustc. This may not behave well.
Copying stage1 rustc from stage1 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Warning: Using a potentially old codegen backend. This may not behave well.
Assembling stage2 compiler (x86_64-apple-darwin)
Warning: Using a potentially old librustc. This may not behave well.
Copying stage2 rustc from stage2 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
thread 'main' panicked at 'File::open(stamp) failed with No such file or directory (os error 2)', bootstrap/lib.rs:1109:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: /Users/tinco/Source/rust/build/bootstrap/debug/bootstrap test --keep-stage 1 --stage 2 src/test/pretty
Build completed unsuccessfully in 0:00:03
