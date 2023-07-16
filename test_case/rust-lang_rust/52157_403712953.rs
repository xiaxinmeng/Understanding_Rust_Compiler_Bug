plain
[00:23:25]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:23:25]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:23:25]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:23:25] [RUSTC-TIMING] std_unicode test:false 0.062
[00:23:25] error[E0658]: The attribute `target_vendor` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
[00:23:25]    --> rustc/libc_shim/../../liblibc/src/unix/mod.rs:313:42
[00:23:25]     |
[00:23:25] 313 |         #[cfg_attr(feature = "stdbuild", target_vendor = "rumprun")]
[00:23:25]     |
[00:23:25]     |
[00:23:25]     = help: add #![feature(custom_attribute)] to the crate attributes to enable
[00:23:25] error: aborting due to previous error
[00:23:25] 
[00:23:25] For more information about this error, try `rustc --explain E0658`.
[00:23:25] [RUSTC-TIMING] libc test:false 0.281
[00:23:25] [RUSTC-TIMING] libc test:false 0.281
[00:23:25] error: Could not compile `libc`.
[00:23:25] Caused by:
[00:23:25] Caused by:
[00:23:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name libc rustc/libc_shim/../../liblibc/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 --cfg feature="default" --cfg feature="stdbuild" -C metadata=f5b8757d317d0230 -C extra-filename=-f5b8757d317d0230 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps --target x86_64-unknown-netbsd -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/libcompiler_builtins-5d3c8e5b32757e07.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/libcore-d2548b8f6f05a2b3.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/build/compiler_builtins-7c3380c919672e95/out` (exit code: 101)
[00:23:31] [RUSTC-TIMING] alloc test:false 5.895
[00:23:31] error: build failed
[00:23:31] error: build failed
[00:23:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-netbsd" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:31] expected success, got: exit code: 101
[00:23:31] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:23:31] travis_fold:end:stage1-std

[00:23:31] travis_time:end:stage1-std:start=1531203412015802341,finish=1531203475107645919,duration=63091843578

---
travis_time:end:0f589ba9:start=1531203475778754059,finish=1531203475785643246,duration=6889187
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18cb48e3
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01f26fc3
$ dmesg | grep -i kill
