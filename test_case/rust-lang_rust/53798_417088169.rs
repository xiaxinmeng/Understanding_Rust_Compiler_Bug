plain
[00:25:39]    |         ^^^^
[00:25:39]    |
[00:25:39]    = note: `-D unused-imports` implied by `-D warnings`
[00:25:39] 
[00:25:39] error: unused import: `sys::os::errno`
[00:25:39]   --> libstd/sys/unix/rand.rs:33:9
[00:25:39]    |
[00:25:39] 33 |     use sys::os::errno;
[00:25:39] 
[00:25:43] error: aborting due to 2 previous errors
[00:25:43] 
[00:25:43] [RUSTC-TIMING] std test:false 5.471
[00:25:43] [RUSTC-TIMING] std test:false 5.471
[00:25:43] The following warnings were emitted during compilation:
[00:25:43] warning: ../libbacktrace/backtrace.c: In function 'unwind':
[00:25:43] warning: ../libbacktrace/backtrace.c:76:6: warning: assignment makes integer from pointer without a cast [enabled by default]
[00:25:43] warning:    pc = _Unwind_GetIP (context);
[00:25:43] warning:       ^
[00:25:43] warning:       ^
[00:25:43] 
[00:25:43] error: Could not compile `std`.
[00:25:43] 
[00:25:43] Caused by:
[00:25:43]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=ae62a46d23b93ea6 -C extra-filename=-ae62a46d23b93ea6 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps --target x86_64-unknown-netbsd -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/liballoc-eea75734c4ba7fe5.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/liballoc_jemalloc-926cbdc3245889a5.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/liballoc_system-5cde7ebd9d10e78c.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/libcompiler_builtins-efc64484d54820ea.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/libcore-22f899c6622a55d2.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/liblibc-913699c741fedca4.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/libpanic_abort-35ec611cae2b287c.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/libpanic_unwind-00db20d78b1a1300.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/libunwind-e3332da9bccb8b0b.rlib -L native=/checkout/obj/build/x86_64-unknown-netbsd/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-netbsd/native/libbacktrace -l static=backtrace -l static=backtrace -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/build/compiler_builtins-32fd61cdd3e8e009/out -L native=/checkout/obj/build/x86_64-unknown-netbsd/native/jemalloc/lib` (exit code: 1)
[00:25:43] expected success, got: exit code: 101
[00:25:43] expected success, got: exit code: 101
[00:25:43] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:25:43] travis_fold:end:stage1-std

[00:25:43] travis_time:end:stage1-std:start=1535573220834843097,finish=1535573276687647328,duration=55852804231

---
travis_time:end:004e2faf:start=1535573277612006597,finish=1535573277618966304,duration=6959707
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00189987
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03dc68a8
travis_time:start:03dc68a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03c2ffb2
$ dmesg | grep -i kill
