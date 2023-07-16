
[00:19:06] Assembling stage1 compiler (x86_64-unknown-linux-gnu)
stage1-std
28.95sBuilding stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:19:07]    Compiling cfg-if v0.1.2
[00:19:07]    Compiling cc v1.0.4
[00:19:07]    Compiling libc v0.2.36
[00:19:07]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:19:07]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:19:08]    Compiling filetime v0.1.15
[00:19:10]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:19:12]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:19:13]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:19:13]    Compiling cmake v0.1.29
[00:19:16]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:19:18]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:19:19]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:19:20]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:19:20]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:19:35] LLVM ERROR: ThinLTO not available
[00:19:35] error: Could not compile `core`.
[00:19:35] 
[00:19:35] Caused by:
[00:19:35]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=488df6a35606ef3a -C extra-filename=-488df6a35606ef3a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[00:19:35] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:19:35] expected success, got: exit code: 101', bootstrap/compile.rs:881:9
[00:19:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
