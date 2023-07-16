plain
[01:09:10]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[01:09:10] [RUSTC-TIMING] panic_unwind test:false 0.232
[01:09:10] warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-redox`
[01:09:10] 
[01:09:12] error[E0063]: missing field `__align` in initializer of `sys::redox::net::netc::in6_addr`
[01:09:12]     |
[01:09:12]     |
[01:09:12] 876 |             inner: c::in6_addr {
[01:09:12]     |                    ^^^^^^^^^^^ missing `__align`
[01:09:13] error: aborting due to previous error
[01:09:13] 
[01:09:13] For more information about this error, try `rustc --explain E0063`.
[01:09:13] [RUSTC-TIMING] std test:false 3.500
[01:09:13] [RUSTC-TIMING] std test:false 3.500
[01:09:13] error: Could not compile `std`.
[01:09:13] 
[01:09:13] Caused by:
[01:09:13]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=3164996808ac7c88 -C extra-filename=-3164996808ac7c88 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps --target x86_64-unknown-redox -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc-6b88bd026f0b6fd2.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc_jemalloc-3c198b5307645456.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc_system-07bf8c34a810c3e2.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libcompiler_builtins-219e93fd3c4ee4c1.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libcore-162918d8b5feb43e.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liblibc-be42a9654e233867.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libpanic_abort-1feda89fc1614f90.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libpanic_unwind-45fc4243d6830344.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libunwind-4b4ebbb764e9c64d.rlib -L native=/checkout/obj/build/x86_64-unknown-redox/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-redox/native/libbacktrace -l static=backtrace -l static=backtrace -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/build/compiler_builtins-ffd29c52e8beac87/out` (exit code: 1)
[01:09:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-redox" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:09:13] expected success, got: exit code: 101
[01:09:13] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1118:9
[01:09:13] travis_fold:end:stage2-std

[01:09:13] travis_time:end:stage2-std:start=1533346959523045075,finish=1533347001612496862,duration=42089451787

---
travis_time:end:0a4ceb51:start=1533347004522695155,finish=1533347004529581171,duration=6886016
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b58d2fa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04fe2c49
travis_time:start:04fe2c49
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:294ba225
$ dmesg | grep -i kill
