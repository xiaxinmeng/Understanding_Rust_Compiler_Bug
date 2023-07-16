plain
travis_time:end:0210e938:start=1548808306589459628,finish=1548808380509102136,duration=73919642508
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:55] 
[01:10:55] running 119 tests
[01:11:20] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:25] i......iii.i.....ii
[01:11:25] 
[01:11:25]  finished in 29.470
[01:11:25] travis_fold:end:test_debuginfo

---
[01:18:17]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[01:18:25] error[E0382]: use of moved value: `v`
[01:18:25]     --> src/liballoc/../liballoc/tests/vec_deque.rs:1466:21
[01:18:25]      |
[01:18:25] 1466 |             let r = v[n];
[01:18:25]      |                     ^ value used here after move
[01:18:25] ...
[01:18:25] 1471 |         assert_eq!(Ok::<_, ()>(66), v.into_iter().try_fold(0, |a, b| Ok(a + b)));
[01:18:25]      |                                     - value moved here
[01:18:25]      |
[01:18:25]      = note: move occurs because `v` has type `std::collections::VecDeque<usize>`, which does not implement the `Copy` trait
[01:18:25] error[E0382]: use of moved value: `v`
[01:18:25]     --> src/liballoc/../liballoc/tests/vec_deque.rs:1467:13
[01:18:25]      |
[01:18:25]      |
[01:18:25] 1467 |             v.rotate_left(n ^ r);
[01:18:25]      |             ^ value used here after move
[01:18:25] ...
[01:18:25] 1471 |         assert_eq!(Ok::<_, ()>(66), v.into_iter().try_fold(0, |a, b| Ok(a + b)));
[01:18:25]      |                                     - value moved here
[01:18:25]      |
[01:18:25]      = note: move occurs because `v` has type `std::collections::VecDeque<usize>`, which does not implement the `Copy` trait
[01:18:25] error[E0382]: use of moved value: `v`
[01:18:25]     --> src/liballoc/../liballoc/tests/vec_deque.rs:1469:13
[01:18:25]      |
[01:18:25]      |
[01:18:25] 1469 |             v.rotate_right(n % 11);
[01:18:25]      |             ^ value used here after move
[01:18:25] 1470 |         }
[01:18:25] 1471 |         assert_eq!(Ok::<_, ()>(66), v.into_iter().try_fold(0, |a, b| Ok(a + b)));
[01:18:25]      |                                     - value moved here
[01:18:25]      |
[01:18:25]      = note: move occurs because `v` has type `std::collections::VecDeque<usize>`, which does not implement the `Copy` trait
[01:18:25] error[E0382]: use of moved value: `v`
[01:18:25]     --> src/liballoc/../liballoc/tests/vec_deque.rs:1471:37
[01:18:25]      |
[01:18:25]      |
[01:18:25] 1471 |         assert_eq!(Ok::<_, ()>(66), v.into_iter().try_fold(0, |a, b| Ok(a + b)));
[01:18:25]      |                                     ^ value moved here in previous iteration of loop
[01:18:25]      |
[01:18:25]      = note: move occurs because `v` has type `std::collections::VecDeque<usize>`, which does not implement the `Copy` trait
[01:18:25] error: aborting due to 4 previous errors
[01:18:25] 
[01:18:25] For more information about this error, try `rustc --explain E0382`.
[01:18:25] error: Could not compile `alloc`.
[01:18:25] error: Could not compile `alloc`.
[01:18:25] warning: build failed, waiting for other jobs to finish...
[01:18:30] error: build failed
[01:18:30] 
[01:18:30] 
[01:18:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:18:30] 
[01:18:30] 
[01:18:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:30] Build completed unsuccessfully in 0:18:55
[01:18:30] Build completed unsuccessfully in 0:18:55
[01:18:30] make: *** [check] Error 1
[01:18:30] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00f8ba6a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 30 01:51:39 UTC 2019
---
travis_time:end:15bb8a90:start=1548813101412580926,finish=1548813101417162231,duration=4581305
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03e7cc44
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:023a0d52
travis_time:start:023a0d52
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02548aa6
$ dmesg | grep -i kill
