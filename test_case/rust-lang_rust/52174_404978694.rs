
$ ./x.py build --keep-stage 1
Updating only changed submodules
Submodules updated in 0.03 seconds
    Finished dev [unoptimized] target(s) in 0.17s
Warning: Using a potentially old librustc. This may not behave well.
Warning: Using a potentially old libstd. This may not behave well.
Warning: Using a potentially old libtest. This may not behave well.
Warning: Using a potentially old librustc. This may not behave well.
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.17s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.16s
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_mir v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_mir)
   Compiling rustc_lint v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_lint)
   Compiling rustc_passes v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_passes)
   Compiling rustc_codegen_utils v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_codegen_utils)
   Compiling rustc_borrowck v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_borrowck)
   Compiling rustc_driver v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_driver)
   Compiling rustc-main v0.0.0 (file:///home/r/src/rust/rustc/src/rustc)
    Finished release [optimized] target(s) in 13.38s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
   Compiling rustc_codegen_llvm v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_codegen_llvm)
    Finished release [optimized] target(s) in 6.31s
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Warning: Using a potentially old librustc. This may not behave well.
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
   Compiling rustc_mir v0.0.0 (file:///home/r/src/rust/rustc/src/librustc_mir)
error[E0463]: can't find crate for `std`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: Could not compile `rustc_mir`.

Caused by:
  process didn't exit successfully: `/home/r/src/rust/rustc/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8e9da67c28005143 -C extra-filename=-8e9da67c28005143 --out-dir /home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-417926933fe788a8.so --extern bitflags=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7a49a1619dd10bb0.rlib --extern byteorder=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-4ca9f7fb66199358.rlib --extern either=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-bdd04aa2a3a1f20d.rlib --extern graphviz=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-b71a70c8a4ed7469.so --extern log=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-0eb143a377f6f226.rlib --extern log_settings=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-6bf5cebbd6300a28.rlib --extern polonius_engine=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-9a343edcb4233c18.rlib --extern rustc=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-efa0afc4091e90c9.so --extern rustc_apfloat=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-ac01335060c9f611.rlib --extern rustc_data_structures=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-80c75f7b514397ee.so --extern rustc_errors=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-2464ee6ca699f10d.so --extern rustc_target=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-635e8fb1260aff1c.so --extern serialize=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-6ea93882817b1dda.so --extern serialize=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-6ea93882817b1dda.rlib --extern syntax=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-ac5f2f7cd567ecc7.so --extern syntax_pos=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-358f903881d0285f.so -L native=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-daf5c28be2095534/out -L native=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-5b64b3ed0b67f8cb/out` (exit code: 101)
command did not execute successfully: "/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "8" "--release" "--manifest-path" "/home/r/src/rust/rustc/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
expected success, got: exit code: 101
thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: /home/r/src/rust/rustc/build/bootstrap/debug/bootstrap build --keep-stage 1
Build completed unsuccessfully in 0:00:21
