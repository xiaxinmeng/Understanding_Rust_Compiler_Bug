plain
    100% |████████████████████████████████| 1.3MB 930kB/s 
Collecting botocore==1.10.54 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2c/84/ca1e66a4c87afdac3ca0dc720e3907e94526947d5094faf8704c0eedaa67/botocore-1.10.54-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 44.0MB/s eta 0:00:01
    0% |▏                               | 20kB 44.0MB/s eta 0:00:01
    0% |▎                               | 30kB 49.6MB/s eta 0:00:01
    0% |▎                               | 40kB 36.3MB/s eta 0:00:01
---
[00:04:24]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:04:25]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:04:25]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:29]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:34] warning: unused return value of `core::sync::atomic::AtomicUsize::compare_and_swap` which must be used
[00:04:34]    --> libstd/sync/mpsc/oneshot.rs:187:21
[00:04:34]     |
[00:04:34] 187 |                     self.state.compare_and_swap(DATA, EMPTY, Ordering::SeqCst);
[00:04:34]     |
[00:04:34]     = note: #[warn(unused_must_use)] on by default
[00:04:34]     = note: #[warn(unused_must_use)] on by default
[00:04:34]     = note: if compare_and_swap does not return `current`, the stored value was not changed
[00:04:41]     Finished release [optimized] target(s) in 44.01s
[00:04:41] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:04:41] travis_fold:end:stage0-std

---
[00:19:49]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:19:50]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:19:50]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:19:55]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:20:02] error: unused return value of `core::sync::atomic::AtomicUsize::compare_and_swap` which must be used
[00:20:02]    --> libstd/sync/mpsc/oneshot.rs:187:21
[00:20:02]     |
[00:20:02] 187 |                     self.state.compare_and_swap(DATA, EMPTY, Ordering::SeqCst);
[00:20:02]     |
[00:20:02]     = note: `-D unused-must-use` implied by `-D warnings`
[00:20:02]     = note: `-D unused-must-use` implied by `-D warnings`
[00:20:02]     = note: if compare_and_swap does not return `current`, the stored value was not changed
[00:20:02] error: aborting due to previous error
[00:20:02] 
[00:20:02] error: Could not compile `std`.
[00:20:02] 
[00:20:02] 
[00:20:02] Caused by:
[00:20:02]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=5bc3f4baef690e48 -C extra-filename=-5bc3f4baef690e48 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-258b2707ebbce886.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-d9a4aa4e4efab0a9.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-123fd80138fcab30.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-86466043f402dfc1.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu//obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
70300 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
68792 ./src/llvm/lib
65420 ./src/llvm-emscripten/test/CodeGen
65160 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
