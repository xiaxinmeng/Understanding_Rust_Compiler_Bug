plain
[00:20:15]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:20:16]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:20:17]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:20:21]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:20:27] error[E0597]: borrowed value does not live long enough
[00:20:27]     |
[00:20:27]     |
[00:20:27] 223 |            LOCAL_STDERR.with(|slot| {
[00:20:27]     |            ^^^^^^^^^^^^ temporary value does not live long enough
[00:20:27] 224 |                *slot.borrow_mut() = s.take();
[00:20:27] 225 |            });
[00:20:27]     |              - temporary value only lives until here
[00:20:27]     |
[00:20:27]     = note: borrowed value must be valid for the static lifetime...
[00:20:27]     = note: consider using a `let` binding to increase its lifetime
[00:20:27] 
[00:20:27] error[E0597]: borrowed value does not live long enough
[00:20:27]     |
[00:20:27] 433 |                 None => &(),
[00:20:27]     |                          ^-
[00:20:27]     |                          ||
[00:20:27]     |                          ||
[00:20:27]     |                          |temporary value only lives until here
[00:20:27]     |                          temporary value does not live long enough
[00:20:27]     |
[00:20:27] note: borrowed value must be valid for the anonymous lifetime #1 defined on the method body at 430:9...
[00:20:27]     |
[00:20:27]     |
[00:20:27] 430 | /         fn get(&mut self) -> &(Any + Send) {
[00:20:27] 431 | |             match self.inner {
[00:20:27] 432 | |                 Some(ref a) => a,
[00:20:27] 433 | |                 None => &(),
[00:20:27] 435 | |         }
[00:20:27]     | |_________^
[00:20:27] 
[00:20:27] 
iler_builtins-30cc8e27b6fe75eb.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-9d37d95f0c4f2954.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-f8b3dd8e60562e7e.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-d58b9a962a6b502c.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-44d3e8b90f7d78f1.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-457ec772a506b6ac.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0226c7120ae54e2a.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-faa081c0f92f0e3b.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-f430c0b2bfd2c0fa.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-e5529cb27937bb7c.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-61a63a2307487562.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-0a2e62b135669011/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:20:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:28] expected success, got: exit code: 101
[00:20:28] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1114:9
[00:20:28] travis_fold:end:stage1-std

[00:20:28] travis_time:end:stage1-std:start=1530714635005514670,finish=1530714709494729621,duration=74489214951


[00:20:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:28] Build completed unsuccessfully in 0:15:50
[00:20:28] Makefile:28: recipe for target 'all' failed
[00:20:28] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17e6c31e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
