
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "4" "--target" "x86_64-unknown-linux-gnu" "-v" "--release" "--locked" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
       Fresh libc v0.2.21
       Fresh core v0.0.0 (file:///checkout/src/libcore)
       Fresh filetime v0.1.10
       Fresh gcc v0.3.45
       Fresh rand v0.0.0 (file:///checkout/src/librand)
       Fresh alloc v0.0.0 (file:///checkout/src/liballoc)
       Fresh cmake v0.1.22
       Fresh libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
       Fresh build_helper v0.1.0 (file:///checkout/src/build_helper)
       Fresh std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
       Fresh collections v0.0.0 (file:///checkout/src/libcollections)
       Fresh panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
       Fresh unwind v0.0.0 (file:///checkout/src/libunwind)
   Compiling compiler_builtins v0.0.0 (file:///checkout/src/libcompiler_builtins)
       Fresh panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
     Running `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiler_builtins /checkout/src/libcompiler_builtins/lib.rs --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=863b57a66ba6c3e1 -C extra-filename=-863b57a66ba6c3e1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-bfaa82017ca17cb2.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/compiler-rt/. -l static=compiler-rt`
       Fresh alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
       Fresh rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
       Fresh rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
       Fresh alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
       Fresh rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
       Fresh rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
error: could not find native static library `compiler-rt`, perhaps an -L flag is missing?

error: Could not compile `compiler_builtins`.

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiler_builtins /checkout/src/libcompiler_builtins/lib.rs --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=863b57a66ba6c3e1 -C extra-filename=-863b57a66ba6c3e1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-bfaa82017ca17cb2.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/compiler-rt/. -l static=compiler-rt` (exit code: 101)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "4" "--target" "x86_64-unknown-linux-gnu" "-v" "--release" "--locked" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
expected success, got: exit code: 101

