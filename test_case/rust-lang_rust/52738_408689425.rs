plain
[01:03:11] 
[01:03:11] error[E0433]: failed to resolve. Use of undeclared type or module `iter`
[01:03:11]    --> libstd/sys/redox/process.rs:299:37
[01:03:11]     |
[01:03:11] 299 |         let args: Vec<[usize; 2]> = iter::once(
[01:03:11]     |                                     ^^^^ Use of undeclared type or module `iter`
[01:03:14] error: aborting due to previous error
[01:03:14] 
[01:03:14] For more information about this error, try `rustc --explain E0433`.
[01:03:14] [RUSTC-TIMING] std test:false 3.442
[01:03:14] [RUSTC-TIMING] std test:false 3.442
[01:03:14] error: Could not compile `std`.
[01:03:14] 
[01:03:14] Caused by:
[01:03:14]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=5acb4a772c226432 -C extra-filename=-5acb4a772c226432 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps --target x86_64-unknown-redox -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc-54de1e6e18170c67.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc_jemalloc-53666a930e3ad04f.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liballoc_system-42b58af3303ea433.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libcompiler_builtins-77c89b8f8fd767a0.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libcore-2706c4a39e374c53.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/liblibc-c218be5828e2f028.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libpanic_abort-4a8dc0e01b2e80eb.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libpanic_unwind-ec538ede862f0199.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libstd_unicode-ee9c3372b170ad3f.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/deps/libunwind-7481e04d1a7a227f.rlib -L native=/checkout/obj/build/x86_64-unknown-redox/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-redox/native/libbacktrace -l static=backtrace -l static=backtrace -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-redox/release/build/compiler_builtins-3dbe2456c9d3a72d/out` (exit code: 1)
[01:03:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-redox" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:03:14] expected success, got: exit code: 101
[01:03:14] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[01:03:14] travis_fold:end:stage2-std

[01:03:14] travis_time:end:stage2-std:start=1532881947584942148,finish=1532881987197051641,duration=39612109493

---
travis_time:end:0237a2ce:start=1532881989526454383,finish=1532881989534908820,duration=8454437
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26c40e98
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:35feb0b4
travis_time:start:35feb0b4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03a8a830
$ dmesg | grep -i kill
