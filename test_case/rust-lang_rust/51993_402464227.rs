plain
[00:19:59]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:20:00]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:20:00]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:20:05]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]     |
[00:20:11]     |
[00:20:11] 701 |     print_to(args, &LOCAL_STDOUT, stdout, "stdout");
[00:20:11]     |                     ^^^^^^^^^^^^                   - temporary value only lives until here
[00:20:11]     |                     |
[00:20:11]     |                     temporary value does not live long enough
[00:20:11]     |
[00:20:11]     = note: borrowed value must be valid for the static lifetime...
[00:20:11]     = note: consider using a `let` binding to increase its lifetime
[00:20:11] 
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]     |
[00:20:11]     |
[00:20:11] 710 |     print_to(args, &LOCAL_STDERR, stderr, "stderr");
[00:20:11]     |                     ^^^^^^^^^^^^                   - temporary value only lives until here
[00:20:11]     |                     |
[00:20:11]     |                     temporary value does not live long enough
[00:20:11]     |
[00:20:11]     = note: borrowed value must be valid for the static lifetime...
[00:20:11]     = note: consider using a `let` binding to increase its lifetime
[00:20:11] 
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]   --> libstd/future.rs:61:9
[00:20:11]    |
[00:20:11] 61 |         TLS_CX.with(|tls_cx| {
[00:20:11]    |         ^^^^^^ temporary value does not live long enough
[00:20:11] 62 |             tls_cx.set(self.0.take());
[00:20:11] 63 |         });
[00:20:11]    |           - temporary value only lives until here
[00:20:11]    |
[00:20:11]    = note: borrowed value must be valid for the static lifetime...
[00:20:11]    = note: consider using a `let` binding to increase its lifetime
[00:20:11] 
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]   --> libstd/future.rs:73:18
[00:20:11]    |
[00:20:11] 73 |     let old_cx = TLS_CX.with(|tls_cx| {
[00:20:11]    |                  ^^^^^^ temporary value does not live long enough
[00:20:11] 80 |     });
[00:20:11] 80 |     });
[00:20:11]    |       - temporary value only lives until here
[00:20:11]    |
[00:20:11]    = note: borrowed value must be valid for the static lifetime...
[00:20:11]    = note: consider using a `let` binding to increase its lifetime
[00:20:11] 
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]    --> libstd/future.rs:96:18
[00:20:11]     |
[00:20:11] 96  |     let cx_ptr = TLS_CX.with(|tls_cx| {
[00:20:11]     |                  ^^^^^^ temporary value does not live long enough
[00:20:11] 100 |     });
[00:20:11] 100 |     });
[00:20:11]     |       - temporary value only lives until here
[00:20:11]     |
[00:20:11]     = note: borrowed value must be valid for the static lifetime...
[00:20:11]     = note: consider using a `let` binding to increase its lifetime
[00:20:11] 
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]    |
[00:20:11]    |
[00:20:11] 47 |     THREAD_INFO.with(|c| assert!(c.borrow().is_none()));
[00:20:11]    |     ^^^^^^^^^^^                                        - temporary value only lives until here
[00:20:11]    |     |
[00:20:11]    |     temporary value does not live long enough
[00:20:11]    |
[00:20:11]    = note: borrowed value must be valid for the static lifetime...
[00:20:11]    = note: consider using a `let` binding to increase its lifetime
[00:20:11] 
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]    |
[00:20:11]    |
[00:20:11] 48 |     THREAD_INFO.with(move |c| *c.borrow_mut() = Some(ThreadInfo{
[00:20:11]    |     ^^^^^^^^^^^ temporary value does not live long enough
[00:20:11] 51 |     }));
[00:20:11] 51 |     }));
[00:20:11]    |        - temporary value only lives until here
[00:20:11]    |
[00:20:11]    = note: borrowed value must be valid for the static lifetime...
[00:20:11]    = note: consider using a `let` binding to increase its lifetime
[00:20:11] 
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]    |
[00:20:11]    |
[00:20:11] 55 |     THREAD_INFO.with(move |c| c.borrow_mut().as_mut().unwrap().stack_guard = stack_guard);
[00:20:11]    |     ^^^^^^^^^^^ temporary value does not live long enough                                - temporary value only lives until here
[00:20:11]    |
[00:20:11]    = note: borrowed value must be valid for the static lifetime...
[00:20:11]    = note: consider using a `let` binding to increase its lifetime
[00:20:11] 
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]     |
[00:20:11]     |
[00:20:11] 218 |     let prev = LOCAL_STDERR.with(|s| s.borrow_mut().take());
[00:20:11]     |                ^^^^^^^^^^^^                                - temporary value only lives until here
[00:20:11]     |                |
[00:20:11]     |                temporary value does not live long enough
[00:20:11]     |
[00:20:11]     = note: borrowed value must be valid for the static lifetime...
[00:20:11]     = note: consider using a `let` binding to increase its lifetime
[00:20:11] 
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]     |
[00:20:11]     |
[00:20:11] 223 |            LOCAL_STDERR.with(|slot| {
[00:20:11]     |            ^^^^^^^^^^^^ temporary value does not live long enough
[00:20:11] 224 |                *slot.borrow_mut() = s.take();
[00:20:11] 225 |            });
[00:20:11]     |              - temporary value only lives until here
[00:20:11]     |
[00:20:11]     = note: borrowed value must be valid for the static lifetime...
[00:20:11]     = note: consider using a `let` binding to increase its lifetime
[00:20:11] 
[00:20:11] error[E0597]: borrowed value does not live long enough
[00:20:11]     |
[00:20:11] 433 |                 None => &(),
[00:20:11]     |                          ^-
[00:20:11]     |                          ||
[00:20:11]     |                          ||
[00:20:11]     |                          |temporary value only lives until here
[00:20:11]     |                          temporary value does not live long enough
[00:20:11]     |
[00:20:11] note: borrowed value must be valid for the anonymous lifetime #1 defined on the method body at 430:9...
[00:20:11]     |
[00:20:11]     |
[00:20:11] 430 | /         fn get(&mut self) -> &(Any + Send) {
[00:20:11] 431 | |             match self.inner {
[00:20:11] 432 | |                 Some(ref a) => a,
[00:20:11] 433 | |                 None => &(),
[00:20:11] 435 | |         }
[00:20:11]     | |_________^
[00:20:11] 
bj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-0a2e62b135669011/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
bj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-0a2e62b135669011/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:20:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:11] expected success, got: exit code: 101
[00:20:11] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1114:9
[00:20:11] travis_fold:end:stage1-std

[00:20:11] travis_time:end:stage1-std:start=1530707165801669172,finish=1530707237582668503,duration=71780999331


[00:20:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:11] Build completed unsuccessfully in 0:15:42
[00:20:11] make: *** [all] Error 1
[00:20:11] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:33a84b6f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2bf75e24:start=1530707238231134085,finish=1530707238239295912,duration=8161827
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:000c6730
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:31e2b54b
$ dmesg | grep -i kill
