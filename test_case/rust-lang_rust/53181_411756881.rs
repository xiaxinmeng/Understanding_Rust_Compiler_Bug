
Building stage1 std artifacts (sparcv9-sun-solaris -> sparcv9-sun-solaris)
   Compiling cc v1.0.15
   Compiling core v0.0.0 (file:///scratch/userland-rust/components/rust/rustc/rustc-1.28.0-src/src/libcore)
   Compiling unwind v0.0.0 (file:///scratch/userland-rust/components/rust/rustc/rustc-1.28.0-src/src/libunwind)
   Compiling build_helper v0.1.0 (file:///scratch/userland-rust/components/rust/rustc/rustc-1.28.0-src/src/build_helper)
   Compiling compiler_builtins v0.0.0 (file:///scratch/userland-rust/components/rust/rustc/rustc-1.28.0-src/src/rustc/compiler_builtins_shim)
   Compiling alloc_jemalloc v0.0.0 (file:///scratch/userland-rust/components/rust/rustc/rustc-1.28.0-src/src/liballoc_jemalloc)
   Compiling std v0.0.0 (file:///scratch/userland-rust/components/rust/rustc/rustc-1.28.0-src/src/libstd)
error: Could not compile `core`.

Caused by:
  process didn't exit successfully: `/scratch/userland-rust/components/rust/rustc/build/sparcv9/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=15421603fe6aafcc -C extra-filename=-15421603fe6aafcc --out-dir /scratch/userland-rust/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage1-std/sparcv9-sun-solaris/release/deps --target sparcv9-sun-solaris -L dependency=/scratch/userland-rust/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage1-std/sparcv9-sun-solaris/release/deps -L dependency=/scratch/userland-rust/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage1-std/release/deps` (signal: 10, SIGBUS: access to undefined memory)
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/usr/bin/cargo" "build" "--target" "sparcv9-sun-solaris" "-j" "4" "--release" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/scratch/userland-rust/components/rust/rustc/rustc-1.28.0-src/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: /scratch/userland-rust/components/rust/rustc/build/sparcv9/build/bootstrap/debug/bootstrap build
Build completed unsuccessfully in 1:10:13
