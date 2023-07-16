plain
[00:26:05] [RUSTC-TIMING] rustc_asan test:false 0.149
[00:26:05] [RUSTC-TIMING] rustc_tsan test:false 0.159
[00:26:05] [RUSTC-TIMING] rustc_lsan test:false 0.115
[00:26:05] [RUSTC-TIMING] panic_unwind test:false 0.332
[00:26:22] error: linker `gcc` not found
[00:26:22]   = note: No such file or directory (os error 2)
[00:26:22] 
[00:26:22] error: aborting due to previous error
[00:26:22] 
[00:26:22] 
[00:26:22] [RUSTC-TIMING] std test:false 16.896
[00:26:22] error: Could not compile `std`.
[00:26:22] 
[00:26:22] Caused by:
[00:26:22]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=62a688c82cba17fd -C extra-filename=-62a688c82cba17fd --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-c7be2bc331bb51f3.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-f4c31a1b5069a5d3.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-3a42364137cf8c58.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-59deabe6ea9ee10b.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-43bbc2d3306b6cdc.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-26570100aba92393.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-b303c6364fbd2323.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-2b87030166cb50ad.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-f03be1eefa458464.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-93db53191bbfc2a8.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-4847049976e069e4.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-7d92ee458a79308c.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-94fc4ce946e0b28a.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-ba8d1c81ac6a0709/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 1)

[00:26:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:26:22] expected success, got: exit code: 101
[00:26:22] travis_time:end:stage1-std:start=1534662450017803918,finish=1534662514096203877,duration=64078399959
[00:26:22] travis_time:end:stage1-std:start=1534662450017803918,finish=1534662514096203877,duration=64078399959

[00:26:22] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:26:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host i686-unknown-freebsd --target i686-unknown-freebsd
[00:26:22] Build completed unsuccessfully in 0:22:34
travis_time:end:009cf798:start=1534660931664168133,finish=1534662514349674640,duration=1582685506507

---
travis_time:end:136854e4:start=1534662514932798552,finish=1534662514938984678,duration=6186126
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05fea5bd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06fc432a
travis_time:start:06fc432a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17631400
$ dmesg | grep -i kill
