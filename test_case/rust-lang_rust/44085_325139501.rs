
$ ./x.py build --stage 1
[...]
Building stage1 compiler artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
   Compiling rustc_back v0.0.0 (file:///Users/bjorn/Documents/rust_fork/src/librustc_back)
   Compiling syntax v0.0.0 (file:///Users/bjorn/Documents/rust_fork/src/libsyntax)
   Compiling log v0.3.8
[...]
   Compiling rustc_save_analysis v0.0.0 (file:///Users/bjorn/Documents/rust_fork/src/librustc_save_analysis)
warning: LLVM not supported, so non rlib output type dylib will be treated like rlib libraries

warning: LLVM not supported, so non rlib output type dylib will be treated like rlib libraries

   Compiling rustc-main v0.0.0 (file:///Users/bjorn/Documents/rust_fork/src/rustc)
error: LLVM is not supported by this rustc

error: Could not compile `rustc-main`.

Caused by:
  process didn't exit successfully: `/Users/bjorn/Documents/rust_fork/no_llvm_build/build/bootstrap/debug/rustc --crate-name rustc /Users/bjorn/Documents/rust_fork/src/rustc/rustc.rs --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_back" -C metadata=9c436ad388a4a0ca -C extra-filename=-9c436ad388a4a0ca --out-dir /Users/bjorn/Documents/rust_fork/no_llvm_build/build/x86_64-apple-darwin/stage1-rustc/x86_64-apple-darwin/release/deps --target x86_64-apple-darwin -L dependency=/Users/bjorn/Documents/rust_fork/no_llvm_build/build/x86_64-apple-darwin/stage1-rustc/x86_64-apple-darwin/release/deps -L dependency=/Users/bjorn/Documents/rust_fork/no_llvm_build/build/x86_64-apple-darwin/stage1-rustc/release/deps --extern rustc_back=/Users/bjorn/Documents/rust_fork/no_llvm_build/build/x86_64-apple-darwin/stage1-rustc/x86_64-apple-darwin/release/deps/librustc_back-853a8b36deced665.dylib --extern rustc_driver=/Users/bjorn/Documents/rust_fork/no_llvm_build/build/x86_64-apple-darwin/stage1-rustc/x86_64-apple-darwin/release/deps/librustc_driver-cc8fef39b95e564b.dylib -L native=/Users/bjorn/Documents/rust_fork/no_llvm_build/build/x86_64-apple-darwin/stage1-rustc/x86_64-apple-darwin/release/build/miniz-sys-e7bda6e0ddcea6f1/out` (exit code: 101)
thread 'main' panicked at 'command did not execute successfully: "/Users/bjorn/Documents/rust_fork/no_llvm_build/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "-j" "8" "--target" "x86_64-apple-darwin" "--release" "--features" " jemalloc" "--manifest-path" "/Users/bjorn/Documents/rust_fork/src/rustc/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', /Users/bjorn/Documents/rust_fork/src/bootstrap/compile.rs:883:8
[...]
