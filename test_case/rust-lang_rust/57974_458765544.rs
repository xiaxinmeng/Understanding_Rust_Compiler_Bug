plain
travis_time:end:0f450735:start=1548803812323085264,finish=1548803885287012601,duration=72963927337
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
[01:14:33] 
[01:14:33] running 119 tests
[01:14:58] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:15:03] i......iii.i.....ii
[01:15:03] 
[01:15:03]  finished in 29.242
[01:15:03] travis_fold:end:test_debuginfo

---
[01:22:06]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[01:22:15] error[E0382]: use of moved value: `v`
[01:22:15]     --> src/liballoc/../liballoc/tests/vec_deque.rs:1466:21
[01:22:15]      |
[01:22:15] 1466 |             let r = v[n];
[01:22:15]      |                     ^ value used here after move
[01:22:15] ...
[01:22:15] 1471 |         assert_eq!(Ok::<_, ()>(66), v.into_iter().try_fold(0, |a, b| Ok(a + b)));
[01:22:15]      |                                     - value moved here
[01:22:15]      |
[01:22:15]      = note: move occurs because `v` has type `std::collections::VecDeque<usize>`, which does not implement the `Copy` trait
[01:22:15] error[E0382]: use of moved value: `v`
[01:22:15]     --> src/liballoc/../liballoc/tests/vec_deque.rs:1467:13
[01:22:15]      |
[01:22:15]      |
[01:22:15] 1467 |             v.rotate_left(n ^ r);
[01:22:15]      |             ^ value used here after move
[01:22:15] ...
[01:22:15] 1471 |         assert_eq!(Ok::<_, ()>(66), v.into_iter().try_fold(0, |a, b| Ok(a + b)));
[01:22:15]      |                                     - value moved here
[01:22:15]      |
[01:22:15]      = note: move occurs because `v` has type `std::collections::VecDeque<usize>`, which does not implement the `Copy` trait
[01:22:15] error[E0382]: use of moved value: `v`
[01:22:15]     --> src/liballoc/../liballoc/tests/vec_deque.rs:1469:13
[01:22:15]      |
[01:22:15]      |
[01:22:15] 1469 |             v.rotate_right(n % 11);
[01:22:15]      |             ^ value used here after move
[01:22:15] 1470 |         }
[01:22:15] 1471 |         assert_eq!(Ok::<_, ()>(66), v.into_iter().try_fold(0, |a, b| Ok(a + b)));
[01:22:15]      |                                     - value moved here
[01:22:15]      |
[01:22:15]      = note: move occurs because `v` has type `std::collections::VecDeque<usize>`, which does not implement the `Copy` trait
[01:22:15] error[E0382]: use of moved value: `v`
[01:22:15]     --> src/liballoc/../liballoc/tests/vec_deque.rs:1471:37
[01:22:15]      |
[01:22:15]      |
[01:22:15] 1471 |         assert_eq!(Ok::<_, ()>(66), v.into_iter().try_fold(0, |a, b| Ok(a + b)));
[01:22:15]      |                                     ^ value moved here in previous iteration of loop
[01:22:15]      |
[01:22:15]      = note: move occurs because `v` has type `std::collections::VecDeque<usize>`, which does not implement the `Copy` trait
[01:22:15] error: aborting due to 4 previous errors
[01:22:15] 
[01:22:15] For more information about this error, try `rustc --explain E0382`.
[01:22:15] error: Could not compile `alloc`.
[01:22:15] error: Could not compile `alloc`.
[01:22:15] warning: build failed, waiting for other jobs to finish...
[01:22:20] error: build failed
[01:22:20] 
[01:22:20] 
[01:22:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:22:20] 
[01:22:20] 
[01:22:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:20] Build completed unsuccessfully in 0:19:20
[01:22:20] Build completed unsuccessfully in 0:19:20
[01:22:20] make: *** [check] Error 1
[01:22:20] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02ef8918
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 30 00:40:34 UTC 2019
---
travis_time:end:043f888a:start=1548808835951465266,finish=1548808835956618014,duration=5152748
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:159505c8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03caaf58
travis_time:start:03caaf58
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:196aa5c0
$ dmesg | grep -i kill
