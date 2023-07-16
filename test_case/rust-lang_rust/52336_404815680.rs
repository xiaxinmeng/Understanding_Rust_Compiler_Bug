plain
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:33]    Compiling libc v0.2.42
[01:01:34]    Compiling rand v0.4.2
[01:01:37]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:01:38] error: trait objects without an explicit `dyn` are deprecated
[01:01:38]   --> liballoc/../liballoc/tests/arc.rs:21:21
[01:01:38]    |
[01:01:38] 21 |     let mut a: Weak<Any> = a;  // Unsizing
[01:01:38]    |                     ^^^ help: use `dyn`: `dyn Any`
[01:01:38]    |
[01:01:38]    = note: requested on the command line with `-D bare-trait-objects`
[01:01:38] 
[01:01:38] error: trait objects without an explicit `dyn` are deprecated
[01:01:38]   --> liballoc/../liballoc/tests/arc.rs:42:16
[01:01:38]    |
[01:01:38] 42 |     let a: Arc<Any> = a;  // Unsizing
[01:01:38]    |                ^^^ help: use `dyn`: `dyn Any`
[01:01:38] 
[01:01:38] error: trait objects without an explicit `dyn` are deprecated
[01:01:38]   --> liballoc/../liballoc/tests/arc.rs:52:21
[01:01:38]    |
[01:01:38] 52 |     let mut b: Weak<Any> = b;  // Unsizing
[01:01:38]    |                     ^^^ help: use `dyn`: `dyn Any`
[01:01:38] 
[01:01:38] error: trait objects without an explicit `dyn` are deprecated
[01:01:38]   --> liballoc/../liballoc/tests/btree/set.rs:43:58
[01:01:38]    |
[01:01:38] 43 |     where F: FnOnce(&BTreeSet<i32>, &BTreeSet<i32>, &mut FnMut(&i32) -> bool) -> bool
[01:01:38]    |                                                          ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(&i32) -> bool`
[01:01:38] 
[01:01:38] error: trait objects without an explicit `dyn` are deprecated
[01:01:38]   --> liballoc/../liballoc/tests/rc.rs:21:21
[01:01:38]    |
[01:01:38] 21 |     let mut a: Weak<Any> = a;  // Unsizing
[01:01:38]    |                     ^^^ help: use `dyn`: `dyn Any`
[01:01:38] 
[01:01:38] error: trait objects without an explicit `dyn` are deprecated
[01:01:38]   --> liballoc/../liballoc/tests/rc.rs:42:15
[01:01:38]    |
[01:01:38] 42 |     let a: Rc<Any> = a;  // Unsizing
[01:01:38]    |               ^^^ help: use `dyn`: `dyn Any`
[01:01:38] 
[01:01:38] error: trait objects without an explicit `dyn` are deprecated
[01:01:38]   --> liballoc/../liballoc/tests/rc.rs:52:21
[01:01:38]    |
[01:01:38] 52 |     let mut b: Weak<Any> = b;  // Unsizing
[01:01:38]    |                     ^^^ help: use `dyn`: `dyn Any`
[01:01:38] 
[01:01:38] error: trait objects without an explicit `dyn` are deprecated
[01:01:38]   --> liballoc/../liballoc/tests/lib.rs:66:62
[01:01:38]    |
[01:01:38] 66 |     let mut hasher_2 = Box::new(DefaultHasher::new()) as Box<Hasher>;
[01:01:38]    |                                                              ^^^^^^ help: use `dyn`: `dyn Hasher`
[01:01:44] error: aborting due to 8 previous errors
[01:01:44] 
[01:01:44] error: Could not compile `alloc`.
[01:01:44] warning: build failed, waiting for other jobs to finish...
[01:01:44] warning: build failed, waiting for other jobs to finish...
[01:01:47] error: build failed
[01:01:47] 
[01:01:47] 
[01:01:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:01:47] 
[01:01:47] 
[01:01:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:47] Build completed unsuccessfully in 0:16:50
[01:01:47] Build completed unsuccessfully in 0:16:50
[01:01:47] make: *** [check] Error 1
[01:01:47] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1cb1cbf0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1b605448:start=1531484051280120099,finish=1531484051285410859,duration=5290760
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02b46c28
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a697203
$ dmesg | grep -i kill
