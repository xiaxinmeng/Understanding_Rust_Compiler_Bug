plain
travis_time:end:19a1df78:start=1556716746247055212,finish=1556716831470755227,duration=85223700015
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:14] 
[01:05:14] running 5476 tests
[01:05:17] ........................................................................................F........... 100/5476
[01:05:23] .................................................................................................... 300/5476
[01:05:26] .................................................................................................... 400/5476
[01:05:29] ...........................................................................................i........ 500/5476
[01:05:33] .................................................................................................... 600/5476
---
[01:08:00] .................................................................................................... 4700/5476
[01:08:05] .................................................................................................... 4800/5476
[01:08:08] .................................................................................................... 4900/5476
[01:08:12] .................................................................................................... 5000/5476
[01:08:16] .....................................F..F........................................................... 5100/5476
[01:08:22] .................................................................................................... 5300/5476
[01:08:25] .................................................................................................... 5400/5476
[01:08:27] ..............i.............................................................
[01:08:27] failures:
[01:08:27] failures:
[01:08:27] 
[01:08:27] ---- [ui] ui/associated-types/associated-types-overridden-binding-2.rs stdout ----
[01:08:27] 
[01:08:27] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:08:27] status: exit code: 101
[01:08:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/auxiliary" "-A" "unused"
[01:08:27] ------------------------------------------
[01:08:27] 
[01:08:27] ------------------------------------------
[01:08:27] stderr:
[01:08:27] stderr:
[01:08:27] ------------------------------------------
[01:08:27] error: internal compiler error: src/librustc/hir/map/mod.rs:632: couldn't find hir id HirId { owner: DefIndex(0:0), local_id: 0 } in the HIR map
[01:08:27] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
[01:08:27] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:27] error: aborting due to previous error
[01:08:27] 
[01:08:27] 
[01:08:27] 
[01:08:27] note: the compiler unexpectedly panicked. this is a bug.
[01:08:27] 
[01:08:27] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:08:27] 
[01:08:27] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:08:27] 
[01:08:27] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:08:27] 
[01:08:27] ------------------------------------------
[01:08:27] 
[01:08:27] 
[01:08:27] 
[01:08:27] ---- [ui] ui/traits/trait-alias-ambiguous.rs stdout ----
[01:08:27] 
[01:08:27] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:08:27] status: exit code: 101
[01:08:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/auxiliary" "-A" "unused"
[01:08:27] ------------------------------------------
[01:08:27] 
[01:08:27] ------------------------------------------
[01:08:27] stderr:
[01:08:27] stderr:
[01:08:27] ------------------------------------------
[01:08:27] error: internal compiler error: src/librustc/hir/map/mod.rs:632: couldn't find hir id HirId { owner: DefIndex(0:0), local_id: 0 } in the HIR map
[01:08:27] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
[01:08:27] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:27] error: aborting due to previous error
[01:08:27] 
[01:08:27] 
[01:08:27] 
[01:08:27] note: the compiler unexpectedly panicked. this is a bug.
[01:08:27] 
[01:08:27] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:08:27] 
[01:08:27] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:08:27] 
[01:08:27] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:08:27] 
[01:08:27] ------------------------------------------
[01:08:27] 
[01:08:27] 
[01:08:27] 
[01:08:27] ---- [ui] ui/traits/trait-alias-object.rs stdout ----
[01:08:27] 
[01:08:27] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:08:27] status: exit code: 101
[01:08:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-object/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-object/auxiliary" "-A" "unused"
[01:08:27] ------------------------------------------
[01:08:27] 
[01:08:27] ------------------------------------------
[01:08:27] stderr:
[01:08:27] stderr:
[01:08:27] ------------------------------------------
[01:08:27] error[E0038]: the trait `EqAlias` cannot be made into an object
[01:08:27]    |
[01:08:27]    |
[01:08:27] LL |     let _: &dyn EqAlias = &123; //~ ERROR `EqAlias` cannot be made into an object
[01:08:27]    |             ^^^^^^^^^^^ the trait `EqAlias` cannot be made into an object
[01:08:27]    = note: the trait cannot use `Self` as a type parameter in the supertraits or where-clauses
[01:08:27] 
[01:08:27] 
[01:08:27] error[E0191]: the value of the associated type `Item` (from the trait `std::iter::Iterator`) must be specified
[01:08:27]    |
[01:08:27]    |
[01:08:27] LL |     let _: &dyn IteratorAlias = &vec![123].into_iter(); //~ ERROR must be specified
[01:08:27]    |             ^^^^^^^^^^^^^^^^^ associated type `Item` must be specified
[01:08:27] 
[01:08:27] error: internal compiler error: src/librustc/hir/map/mod.rs:632: couldn't find hir id HirId { owner: DefIndex(0:0), local_id: 0 } in the HIR map
[01:08:27] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
[01:08:27] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:27] error: aborting due to 3 previous errors
[01:08:27] 
---
[01:08:27] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:08:27] 
[01:08:27] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:08:27] 
[01:08:27] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:08:27] 
[01:08:27] ------------------------------------------
[01:08:27] 
[01:08:27] 
---
[01:08:27] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:08:27] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:27] 
[01:08:27] 
[01:08:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:27] 
[01:08:27] 
[01:08:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:27] Build completed unsuccessfully in 0:04:10
[01:08:27] Build completed unsuccessfully in 0:04:10
[01:08:27] make: *** [check] Error 1
[01:08:27] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:098bea48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  1 14:29:07 UTC 2019
---
travis_time:end:0214b825:start=1556720949058923444,finish=1556720949065924552,duration=7001108
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11d18fa8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2e268658
$ dmesg | grep -i kill
