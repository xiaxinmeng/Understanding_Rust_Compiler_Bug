plain
[00:31:59]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:31:59]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:31:59]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:32:00] [RUSTC-TIMING] std_unicode test:false 0.086
[00:32:00] error[E0658]: The attribute `target_vendor` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
[00:32:00]    --> rustc/libc_shim/../../liblibc/src/unix/mod.rs:313:42
[00:32:00]     |
[00:32:00] 313 |         #[cfg_attr(feature = "stdbuild", target_vendor = "rumprun")]
[00:32:00]     |
[00:32:00]     |
[00:32:00]     = help: add #![feature(custom_attribute)] to the crate attributes to enable
[00:32:00] error: aborting due to previous error
[00:32:00] 
[00:32:00] For more information about this error, try `rustc --explain E0658`.
[00:32:00] [RUSTC-TIMING] libc test:false 0.328
[00:32:00] [RUSTC-TIMING] libc test:false 0.328
[00:32:00] error: Could not compile `libc`.
[00:32:00] Caused by:
[00:32:00] Caused by:
[00:32:00]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name libc rustc/libc_shim/../../liblibc/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 --cfg feature="default" --cfg feature="stdbuild" -C metadata=8dd8865d2057534d -C extra-filename=-8dd8865d2057534d --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps --target x86_64-unknown-netbsd -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/libcompiler_builtins-201253f95fc262be.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/libcore-d6429dbfd5d2229b.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/build/compiler_builtins-4ac7974cfefb5381/out` (exit code: 101)
[00:32:05] [RUSTC-TIMING] alloc test:false 5.871
[00:32:05] error: build failed
[00:32:05] error: build failed
[00:32:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-netbsd" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:32:05] expected success, got: exit code: 101
[00:32:05] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:32:05] travis_fold:end:stage1-std

[00:32:05] travis_time:end:stage1-std:start=1531201660993127732,finish=1531201723813709146,duration=62820581414

---
travis_time:end:1f8277c6:start=1531201724553316708,finish=1531201724559925248,duration=6608540
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a6d36b8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08676444
$ dmesg | grep -i kill
