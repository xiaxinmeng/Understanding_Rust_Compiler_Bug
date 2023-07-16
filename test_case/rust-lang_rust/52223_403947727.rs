plain
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:02]    Compiling libc v0.2.42
[00:57:03]    Compiling rand v0.4.2
[00:57:05]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]   --> liballoc/boxed_test.rs:34:32
[00:57:06]    |
[00:57:06] 34 |     let a = Box::new(8) as Box<Any>;
[00:57:06]    |                                ^^^ help: use `dyn`: `dyn Any`
[00:57:06] note: lint level defined here
[00:57:06]   --> liballoc/lib.rs:75:9
[00:57:06]    |
[00:57:06]    |
[00:57:06] 75 | #![deny(bare_trait_objects)]
[00:57:06] 
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]   --> liballoc/boxed_test.rs:35:35
[00:57:06]    |
[00:57:06] 35 |     let b = Box::new(Test) as Box<Any>;
[00:57:06]    |                                   ^^^ help: use `dyn`: `dyn Any`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]   --> liballoc/boxed_test.rs:50:32
[00:57:06]    |
[00:57:06] 50 |     let a = Box::new(8) as Box<Any>;
[00:57:06]    |                                ^^^ help: use `dyn`: `dyn Any`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]   --> liballoc/boxed_test.rs:51:35
[00:57:06]    |
[00:57:06] 51 |     let b = Box::new(Test) as Box<Any>;
[00:57:06]    |                                   ^^^ help: use `dyn`: `dyn Any`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]   --> liballoc/boxed_test.rs:59:32
[00:57:06]    |
[00:57:06] 59 |     let a = Box::new(8) as Box<Any>;
[00:57:06]    |                                ^^^ help: use `dyn`: `dyn Any`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]   --> liballoc/boxed_test.rs:60:35
[00:57:06]    |
[00:57:06] 60 |     let b = Box::new(Test) as Box<Any>;
[00:57:06]    |                                   ^^^ help: use `dyn`: `dyn Any`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]   --> liballoc/boxed_test.rs:68:24
[00:57:06]    |
[00:57:06] 68 |     let a = &EIGHT as &Any;
[00:57:06]    |                        ^^^ help: use `dyn`: `dyn Any`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]   --> liballoc/boxed_test.rs:69:23
[00:57:06]    |
[00:57:06] 69 |     let b = &TEST as &Any;
[00:57:06]    |                       ^^^ help: use `dyn`: `dyn Any`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]    --> liballoc/boxed_test.rs:113:16
[00:57:06]     |
[00:57:06] 113 |     let x: Box<Foo> = Box::new(Bar(17));
[00:57:06]     |                ^^^ help: use `dyn`: `dyn Foo`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]    --> liballoc/boxed_test.rs:118:20
[00:57:06]     |
[00:57:06] 118 |         let y: Box<Foo> = Box::from_raw(p);
[00:57:06]     |                    ^^^ help: use `dyn`: `dyn Foo`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]      |
[00:57:06]      |
[00:57:06] 1577 |         let arc: Arc<Display> = Arc::new(123);
[00:57:06]      |                      ^^^^^^^ help: use `dyn`: `dyn Display`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]      |
[00:57:06]      |
[00:57:06] 1882 |         let b: Box<Display> = box 123;
[00:57:06]      |                    ^^^^^^^ help: use `dyn`: `dyn Display`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]      |
[00:57:06]      |
[00:57:06] 1883 |         let r: Arc<Display> = Arc::from(b);
[00:57:06]      |                    ^^^^^^^ help: use `dyn`: `dyn Display`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]      |
[00:57:06]      |
[00:57:06] 1892 |         let b: Box<Debug> = box ();
[00:57:06]      |                    ^^^^^ help: use `dyn`: `dyn Debug`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]      |
[00:57:06]      |
[00:57:06] 1893 |         let r: Arc<Debug> = Arc::from(b);
[00:57:06]      |                    ^^^^^ help: use `dyn`: `dyn Debug`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]      |
[00:57:06]      |
[00:57:06] 1910 |         let r1: Arc<Any + Send + Sync> = Arc::new(i32::max_value());
[00:57:06]      |                     ^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + Send + Sync`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]      |
[00:57:06]      |
[00:57:06] 1911 |         let r2: Arc<Any + Send + Sync> = Arc::new("abc");
[00:57:06]      |                     ^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Any + Send + Sync`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]     --> liballoc/rc.rs:1557:20
[00:57:06]      |
[00:57:06] 1557 |         let rc: Rc<Display> = Rc::new(123);
[00:57:06]      |                    ^^^^^^^ help: use `dyn`: `dyn Display`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]     --> liballoc/rc.rs:1758:20
[00:57:06]      |
[00:57:06] 1758 |         let b: Box<Display> = box 123;
[00:57:06]      |                    ^^^^^^^ help: use `dyn`: `dyn Display`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]     --> liballoc/rc.rs:1759:19
[00:57:06]      |
[00:57:06] 1759 |         let r: Rc<Display> = Rc::from(b);
[00:57:06]      |                   ^^^^^^^ help: use `dyn`: `dyn Display`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]     --> liballoc/rc.rs:1768:20
[00:57:06]      |
[00:57:06] 1768 |         let b: Box<Debug> = box ();
[00:57:06]      |                    ^^^^^ help: use `dyn`: `dyn Debug`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]     --> liballoc/rc.rs:1769:19
[00:57:06]      |
[00:57:06] 1769 |         let r: Rc<Debug> = Rc::from(b);
[00:57:06]      |                   ^^^^^ help: use `dyn`: `dyn Debug`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]     --> liballoc/rc.rs:1786:20
[00:57:06]      |
[00:57:06] 1786 |         let r1: Rc<Any> = Rc::new(i32::max_value());
[00:57:06]      |                    ^^^ help: use `dyn`: `dyn Any`
[00:57:06] 
[00:57:06] error: trait objects without an explicit `dyn` are deprecated
[00:57:06]     --> liballoc/rc.rs:1787:20
[00:57:06]      |
[00:57:06] 1787 |         let r2: Rc<Any> = Rc::new("abc");
[00:57:06]      |                    ^^^ help: use `dyn`: `dyn Any`
[00:57:09] error: aborting due to 24 previous errors
[00:57:09] 
[00:57:09] error: Could not compile `alloc`.
[00:57:09] warning: build failed, waiting for other jobs to finish...
[00:57:09] warning: build failed, waiting for other jobs to finish...
[00:57:40] error: build failed
[00:57:40] 
[00:57:40] 
[00:57:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[00:57:40] 
[00:57:40] 
[00:57:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:40] Build completed unsuccessfully in 0:15:51
[00:57:40] Build completed unsuccessfully in 0:15:51
[00:57:40] Makefile:58: recipe for target 'check' failed
[00:57:40] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2114ddeb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0cf86e0d:start=1531252893580039068,finish=1531252893587372399,duration=7333331
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c9f42cb
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a67c1ed
$ dmesg | grep -i kill
