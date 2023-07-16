plain
[01:08:56] [RUSTC-TIMING] alloc test:false 2.530
[01:08:56] error: Could not compile `alloc`.
[01:08:56] 
[01:08:56] Caused by:
[01:08:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=29f0a4c2b0209285 -C extra-filename=-29f0a4c2b0209285 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/thumbv6m-none-eabi/release/deps --target thumbv6m-none-eabi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/thumbv6m-none-eabi/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/thumbv6m-none-eabi/release/deps/libcompiler_builtins-0eff4e3891d39af0.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/thumbv6m-none-eabi/release/deps/libcore-02bed33325735a38.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/thumbv6m-none-eabi/release/build/compiler_builtins-f4ddcda5e74d05f6/out` (exit code: 101)
[01:08:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "thumbv6m-none-eabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "c mem" "-p" "alloc" "-p" "compiler_builtins" "-p" "std_unicode" "--manifest-path" "/checkout/src/rustc/compiler_builtins_shim/Cargo.toml" "--message-format" "json"
[01:08:56] expected success, got: exit code: 101
[01:08:56] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[01:08:56] travis_fold:end:stage2-std

[01:08:56] travis_time:end:stage2-std:start=1530920024822083168,finish=1530920062386157973,duration=37564074805

---
travis_time:end:24028b2d:start=1530920064608513636,finish=1530920064615654578,duration=7140942
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04d8da70
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a6fcaf8
$ dmesg | grep -i kill
