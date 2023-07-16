
  rust git:(issue_12590) ✗ ./x.py test --keep-stage 0 --stage 1 src/test/pretty ; say "I am done. I am really done. ... Really"
Updating only changed submodules
Submodules updated in 0.12 seconds
    Finished dev [unoptimized] target(s) in 0.29s
Warning: Using a potentially old librustc. This may not behave well.
Warning: Using a potentially old librustc. This may not behave well.
Warning: Using a potentially old libtest. This may not behave well.
Warning: Using a potentially old librustc. This may not behave well.
Copying stage0 rustc from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 codegen artifacts (x86_64-apple-darwin -> x86_64-apple-darwin, llvm)
   Compiling serialize v0.0.0 (file:///Users/tinco/Source/rust/src/libserialize)
   Compiling rustc_target v0.0.0 (file:///Users/tinco/Source/rust/src/librustc_target)
   Compiling syntax v0.0.0 (file:///Users/tinco/Source/rust/src/libsyntax)
   Compiling rustc v0.0.0 (file:///Users/tinco/Source/rust/src/librustc)
error[E0463]: can't find crate for `std`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: Could not compile `serialize`.

Caused by:
  process didn't exit successfully: `/Users/tinco/Source/rust/build/bootstrap/debug/rustc --crate-name serialize libserialize/lib.rs --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=b5d7af8c38a39a93 -C extra-filename=-b5d7af8c38a39a93 --out-dir /Users/tinco/Source/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps --target x86_64-apple-darwin -L dependency=/Users/tinco/Source/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps -L dependency=/Users/tinco/Source/rust/build/x86_64-apple-darwin/stage0-rustc/release/deps` (exit code: 101)
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/Users/tinco/Source/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--manifest-path" "/Users/tinco/Source/rust/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
expected success, got: exit code: 101
thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: /Users/tinco/Source/rust/build/bootstrap/debug/bootstrap test --keep-stage 0 --stage 1 src/test/pretty
Build completed unsuccessfully in 0:00:03
