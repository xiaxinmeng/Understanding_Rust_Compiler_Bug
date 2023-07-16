plain
[01:06:03] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:03]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:06:06] error: trait objects without an explicit `dyn` are deprecated
[01:06:06]      |
[01:06:06]      |
[01:06:06] 1441 |     fn avoid_copying_the_body<F>(spawnfn: F) where F: FnOnce(Box<Fn() + Send>) {
[01:06:06]      |                                                                  ^^^^^^^^^^^ help: use `dyn`: `dyn Fn() + Send`
[01:06:06] note: lint level defined here
[01:06:06]     --> libstd/lib.rs:224:9
[01:06:06]      |
[01:06:06]      |
[01:06:06] 224  | #![deny(bare_trait_objects)]
[01:06:06] 
[01:06:06] 
[01:06:06] error: trait objects without an explicit `dyn` are deprecated
[01:06:06]      |
[01:06:06]      |
[01:06:06] 1488 |         fn child_no(x: u32) -> Box<Fn() + Send> {
[01:06:06]      |                                    ^^^^^^^^^^^ help: use `dyn`: `dyn Fn() + Send`
[01:06:06] 
[01:06:06] error: trait objects without an explicit `dyn` are deprecated
[01:06:06]      |
[01:06:06]      |
[01:06:06] 1534 |             panic!(box 413u16 as Box<Any + Send>);
[01:06:06]      |                                      ^^^^^^^^^^ help: use `dyn`: `dyn Any + Send`
[01:06:06] 
[01:06:06] error: trait objects without an explicit `dyn` are deprecated
[01:06:06]      |
[01:06:06]      |
[01:06:06] 1537 |                 type T = Box<Any + Send>;
[01:06:06]      |                              ^^^^^^^^^^ help: use `dyn`: `dyn Any + Send`
[01:06:06] 
[01:06:06] error: trait objects without an explicit `dyn` are deprecated
[01:06:06]     |
[01:06:06]     |
[01:06:06] 536 |         let a = &mut a as &mut (Error + 'static);
[01:06:06]     |                                 ^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Error + 'static`
[01:06:06] 
[01:06:06] error: trait objects without an explicit `dyn` are deprecated
[01:06:06]     |
[01:06:06]     |
[01:06:06] 542 |         let a: Box<Error> = Box::new(A);
[01:06:06]     |                    ^^^^^ help: use `dyn`: `dyn Error`
[01:06:06] 
[01:06:07] error: trait objects without an explicit `dyn` are deprecated
[01:06:07]    --> libstd/io/util.rs:226:40
[01:06:07]     |
[01:06:07] 226 |         assert_eq!(copy(&mut r as &mut Read, &mut w as &mut Write).unwrap(), 1 << 17);
[01:06:07]     |                                        ^^^^ help: use `dyn`: `dyn Read`
[01:06:07] 
[01:06:07] error: trait objects without an explicit `dyn` are deprecated
[01:06:07]    --> libstd/io/util.rs:226:61
[01:06:07]     |
[01:06:07] 226 |         assert_eq!(copy(&mut r as &mut Read, &mut w as &mut Write).unwrap(), 1 << 17);
[01:06:07]     |                                                             ^^^^^ help: use `dyn`: `dyn Write`
[01:06:07] 
[01:06:07] error: trait objects without an explicit `dyn` are deprecated
[01:06:07]     |
[01:06:07]     |
[01:06:07] 930 |     fn each_ip(f: &mut FnMut(SocketAddr)) {
[01:06:07]     |                        ^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(SocketAddr)`
[01:06:07] 
[01:06:07] error: trait objects without an explicit `dyn` are deprecated
[01:06:07]     |
[01:06:07]     |
[01:06:07] 829 |     fn each_ip(f: &mut FnMut(SocketAddr, SocketAddr)) {
[01:06:07]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(SocketAddr, SocketAddr)`
[01:06:17] error: aborting due to 10 previous errors
[01:06:17] 
[01:06:17] error: Could not compile `std`.
[01:06:17] 
[01:06:17] 
[01:06:17] To learn more, run the command again with --verbose.
[01:06:17] 
[01:06:17] 
[01:06:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:06:17] 
[01:06:17] 
[01:06:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:17] Build completed unsuccessfully in 0:21:55
[01:06:17] Build completed unsuccessfully in 0:21:55
[01:06:17] Makefile:58: recipe for target 'check' failed
[01:06:17] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27492dc0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:14f9992e:start=1531252947333569348,finish=1531252947344732517,duration=11163169
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00e37960
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00e606c7
$ dmesg | grep -i kill
