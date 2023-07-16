plain
[00:05:11]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:05:12]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:05:12]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:05:15]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:05:18] error[E0277]: can't compare `net::addr::SocketAddrV4` with `net::addr::SocketAddrV4`
[00:05:18]    |
[00:05:18]    |
[00:05:18] 53 |     V4(#[stable(feature = "rust1", since = "1.0.0")] SocketAddrV4),
[00:05:18]    |                                                      ^^^^^^^^^^^^ no implementation for `net::addr::SocketAddrV4 < net::addr::SocketAddrV4` and `net::addr::SocketAddrV4 > net::addr::SocketAddrV4`
[00:05:18]    |
[00:05:18]    = help: the trait `core::cmp::PartialOrd` is not implemented for `net::addr::SocketAddrV4`
[00:05:18]    = note: required by `core::cmp::PartialOrd::partial_cmp`
[00:05:18] 
[00:05:18] error[E0277]: can't compare `net::addr::SocketAddrV6` with `net::addr::SocketAddrV6`
[00:05:18]    |
[00:05:18]    |
[00:05:18] 56 |     V6(#[stable(feature = "rust1", since = "1.0.0")] SocketAddrV6),
[00:05:18]    |                                                      ^^^^^^^^^^^^ no implementation for `net::addr::SocketAddrV6 < net::addr::SocketAddrV6` and `net::addr::SocketAddrV6 > net::addr::SocketAddrV6`
[00:05:18]    |
[00:05:18]    = help: the trait `core::cmp::PartialOrd` is not implemented for `net::addr::SocketAddrV6`
[00:05:18]    = note: required by `core::cmp::PartialOrd::partial_cmp`
[00:05:18] 
[00:05:18] error[E0277]: the trait bound `net::addr::SocketAddrV4: core::cmp::Ord` is not satisfied
[00:05:18]    |
[00:05:18]    |
[00:05:18] 53 |     V4(#[stable(feature = "rust1", since = "1.0.0")] SocketAddrV4),
[00:05:18]    |                                                      ^^^^^^^^^^^^ the trait `core::cmp::Ord` is not implemented for `net::addr::SocketAddrV4`
[00:05:18]    = note: required by `core::cmp::Ord::cmp`
[00:05:18] 
[00:05:18] 
[00:05:18] error[E0277]: the trait bound `net::addr::SocketAddrV6: core::cmp::Ord` is not satisfied
[00:05:18]    |
[00:05:18]    |
[00:05:18] 56 |     V6(#[stable(feature = "rust1", since = "1.0.0")] SocketAddrV6),
[00:05:18]    |                                                      ^^^^^^^^^^^^ the trait `core::cmp::Ord` is not implemented for `net::addr::SocketAddrV6`
[00:05:18]    = note: required by `core::cmp::Ord::cmp`
[00:05:18] 
[00:05:19] error: aborting due to 4 previous errors
[00:05:19] 
[00:05:19] 
[00:05:19] For more information about this error, try `rustc --explain E0277`.
[00:05:19] error: Could not compile `std`.
[00:05:19] 
[00:05:19] Caused by:
[00:05:19]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=c29a53bc81e5db39 -C extra-filename=-c29a53bc81e5db39 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-eb136989b0a4d15b.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-3dc5fad39cb0df5f.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-21bc9e10c694662b.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-02d8b7c22d5a49ce.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-bd44783aadae9ca1.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-fbf6e852b600f064.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-5644a1c79dc26f62.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-f76abd93c66cd824.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-2334c41b897059f9.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-4e5abd09755edb6d.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-c40628b41b616378.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-09ceed76cedbf73e.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-c762ab8d5644db74.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace -l static=backtrace -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-4e55612f101529ad/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 1)
[00:05:19] expected success, got: exit code: 101
[00:05:19] expected success, got: exit code: 101
[00:05:19] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:05:19] travis_fold:end:stage0-std

[00:05:19] travis_time:end:stage0-std:start=1535552029131599318,finish=1535552064150115703,duration=35018516385


[00:05:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:19] Build completed unsuccessfully in 0:00:36
[00:05:19] make: *** [all] Error 1
[00:05:19] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1542035f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0ac420b6:start=1535552064838002479,finish=1535552064845720653,duration=7718174
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:003bd3c8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:003f0450
travis_time:start:003f0450
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2abd9bdc
$ dmesg | grep -i kill
