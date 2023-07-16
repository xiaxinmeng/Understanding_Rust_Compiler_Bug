plain
[00:22:53]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:22:55]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:22:55]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:23:00]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:23:08] error[E0597]: borrowed value does not live long enough
[00:23:08]     |
[00:23:08]     |
[00:23:08] 223 |            LOCAL_STDERR.with(|slot| {
[00:23:08]     |            ^^^^^^^^^^^^ temporary value does not live long enough
[00:23:08] 224 |                *slot.borrow_mut() = s.take();
[00:23:08] 225 |            });
[00:23:08]     |              - temporary value only lives until here
[00:23:08]     |
[00:23:08]     = note: borrowed value must be valid for the static lifetime...
[00:23:08]     = note: consider using a `let` binding to increase its lifetime
[00:23:08] 
[00:23:08] error[E0597]: borrowed value does not live long enough
[00:23:08]     |
[00:23:08] 433 |                 None => &(),
[00:23:08]     |                          ^-
[00:23:08]     |                          ||
[00:23:08]     |                          ||
[00:23:08]     |                          |temporary value only lives until here
[00:23:08]     |                          temporary value does not live long enough
[00:23:08]     |
[00:23:08] note: borrowed value must be valid for the anonymous lifetime #1 defined on the method body at 430:9...
[00:23:08]     |
[00:23:08]     |
[00:23:08] 430 | /         fn get(&mut self) -> &(Any + Send) {
[00:23:08] 431 | |             match self.inner {
[00:23:08] 432 | |                 Some(ref a) => a,
[00:23:08] 433 | |                 None => &(),
[00:23:08] 435 | |         }
[00:23:08]     | |_________^
[00:23:08] 
[00:23:08] error: aborting due to 2 previous errors
[00:23:08] error: aborting due to 2 previous errors
[00:23:08] 
[00:23:08] For more information about this error, try `rustc --explain E0597`.
[00:23:08] error: Could not compile `std`.
[00:23:08] 
[00:23:08] Caused by:
[00:23:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=84c7fbfce53d7ef5 -C extra-filename=-84c7fbfce53d7ef5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-c69d0f052402a17c.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-07d26af630202296.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-4df9e82ac7dca73d.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-30cc8e27b6fe75eb.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-9d37d95f0c4f2954.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-f8b3dd8e60562e7e.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-d58b9a962a6b502c.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-44d3e8b90f7d78f1.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-457ec772a506b6ac.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0226c7120ae54e2a.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-faa081c0f92f0e3b.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-f430c0b2bfd2c0fa.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-e5529cb27937bb7c.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-61a63a2307487562.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-0a2e62b135669011/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:23:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:08] expected success, got: exit code: 101
[00:23:08] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1114:9
[00:23:08] travis_fold:end:stage1-std

[00:23:08] travis_time:end:stage1-std:start=1530712455383138453,finish=1530712537328120176,duration=81944981723


[00:23:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:08] Build completed unsuccessfully in 0:18:10
[00:23:08] make: *** [all] Error 1
[00:23:08] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0240968c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02c5e233:start=1530712538050908960,finish=1530712538061549308,duration=10640348
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18dabfe8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:028ae30b
$ dmesg | grep -i kill
