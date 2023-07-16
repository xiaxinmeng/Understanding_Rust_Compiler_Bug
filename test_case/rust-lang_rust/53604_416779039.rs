plain
[00:59:55]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:59:55] [RUSTC-TIMING] panic_unwind test:false 0.266
[00:59:55] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-cloudabi`
[00:59:55] 
[01:00:00] error: can only call other `min_const_fn` within a `min_const_fn`
[01:00:00]   --> libstd/sys/cloudabi/condvar.rs:34:38
[01:00:00]    |
[01:00:00] 34 |             condvar: UnsafeCell::new(AtomicU32::new(abi::CONDVAR_HAS_NO_WAITERS.0)),
[01:00:00] 
[01:00:00] 
[01:00:00] error: can only call other `min_const_fn` within a `min_const_fn`
[01:00:00]   --> libstd/sys/cloudabi/rwlock.rs:38:35
[01:00:00]    |
[01:00:00] 38 |             lock: UnsafeCell::new(AtomicU32::new(abi::LOCK_UNLOCKED.0)),
[01:00:00] 
[01:00:01] error: aborting due to 2 previous errors
[01:00:01] 
[01:00:01] [RUSTC-TIMING] std test:false 6.506
[01:00:01] [RUSTC-TIMING] std test:false 6.506
[01:00:01] error: Could not compile `std`.
[01:00:01] 
[01:00:01] Caused by:
[01:00:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=aa5d7a8ad48b9423 -C extra-filename=-aa5d7a8ad48b9423 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps --target x86_64-unknown-cloudabi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liballoc-e6b09e7981e5c6d0.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liballoc_jemalloc-92a552dd3f4923ce.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liballoc_system-044fecd26c422c59.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libcompiler_builtins-1a078d299f90cb45.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libcore-78f485cde79cca86.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/liblibc-79dcada64711b815.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libpanic_abort-c7ed3968cedc70b9.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libpanic_unwind-10530130e67c3d95.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/deps/libunwind-aa94961e054a4260.rlib -l unwind -l c -l compiler_rt -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-cloudabi/release/build/compiler_builtins-30e71f71be0e2543/out` (exit code: 1)

[01:00:01] travis_time:end:stage2-std:start=1535500867134985964,finish=1535500914968851484,duration=47833865520

[01:00:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target x86_64-fuchsia,aarch64-fuchsia,sparcv9-sun-solaris,wasm32-unknown-unknown,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-unknown-cloudabi
[01:00:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target x86_64-fuchsia,aarch64-fuchsia,sparcv9-sun-solaris,wasm32-unknown-unknown,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-unknown-cloudabi
[01:00:01] Build completed unsuccessfully in 0:52:22
[01:00:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-cloudabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:00:01] expected success, got: exit code: 101
[01:00:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
travis_time:end:0e4c5384:start=1535497313018791207,finish=1535500915216592989,duration=3602197801782

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:11b4ece0
---
travis_time:end:174cc98e:start=1535500916484229550,finish=1535500916490549751,duration=6320201
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16cfb9e6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:035c18bc
travis_time:start:035c18bc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b9f5ed0
$ dmesg | grep -i kill
