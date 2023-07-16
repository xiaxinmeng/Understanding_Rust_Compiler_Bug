plain
[01:06:28]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[01:06:29] [RUSTC-TIMING] panic_unwind test:false 0.237
[01:06:29] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-redox`
[01:06:29] 
[01:06:30] error: trait objects without an explicit `dyn` are deprecated
[01:06:30]   --> libstd/sys/redox/process.rs:54:23
[01:06:30]    |
[01:06:30] 54 |     closures: Vec<Box<FnMut() -> io::Result<()> + Send + Sync>>,
[01:06:30]    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut() -> io::Result<()> + Send + Sync`
[01:06:30] note: lint level defined here
[01:06:30]   --> libstd/lib.rs:224:9
[01:06:30]    |
[01:06:30]    |
[01:06:30] 224| #![deny(bare_trait_objects)]
[01:06:30] 
[01:06:30] 
[01:06:30] error: trait objects without an explicit `dyn` are deprecated
[01:06:30]    --> libstd/sys/redox/process.rs:125:31
[01:06:30]     |
[01:06:30] 125 |                        f: Box<FnMut() -> io::Result<()> + Send + Sync>) {
[01:06:30]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut() -> io::Result<()> + Send + Sync`
[01:06:32] error: aborting due to 2 previous errors
[01:06:32] 
[01:06:32] [RUSTC-TIMING] std test:false 3.282
[01:06:32] error: Could not compile `std`.
[01:06:32] error: Could not compile `std`.
[01:06:32] 
[01:06:32] Caused by:
[01:06:32]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=2e8e237846c4d089 -C extra-filename=-2e8e237846c4d089 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps --target x86_64-unknown-redox -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc-5223e952bfc9836f.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc_jemalloc-97d2c134bcb04f15.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc_system-87e7e4f30c314de7.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libcompiler_builtins-7c669ba5e870ed5a.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libcore-33cdf35bf7d87eda.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liblibc-e64d79f4c80deb60.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libpanic_abort-4fdfedbe551ea7b6.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libpanic_unwind-4c945d2b7c6b4d4b.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libstd_unicode-b66524888adce754.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libunwind-3d960dcb5fcea9df.rlib -L native=/checkout/obj/build/x86_64-unknown-redox/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-redox/native/libbacktrace -l static=backtrace -l static=backtrace -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/build/compiler_builtins-b3b8572fe35a60a7/out` (exit code: 101)
[01:06:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-redox" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:06:32] expected success, got: exit code: 101
[01:06:32] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[01:06:32] travis_fold:end:stage2-std

[01:06:32] travis_time:end:stage2-std:start=1531437426166175574,finish=1531437479930530509,duration=53764354935

---
travis_time:end:0a255400:start=1531437482402388042,finish=1531437482409327476,duration=6939434
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b7412d9
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:001dda6e
$ dmesg | grep -i kill
