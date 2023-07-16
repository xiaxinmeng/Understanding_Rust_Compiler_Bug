plain
travis_time:end:1a26b0c4:start=1556633167249527871,finish=1556633168209992226,duration=960464355
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
[01:11:57] 
[01:11:57] running 5472 tests
[01:12:00] ........................................................................................F........... 100/5472
[01:12:06] .................................................................................................... 300/5472
[01:12:09] .................................................................................................... 400/5472
[01:12:13] ...........................................................................................i........ 500/5472
[01:12:17] .................................................................................................... 600/5472
---
[01:14:56] .................................................................................................... 4700/5472
[01:15:01] .................................................................................................... 4800/5472
[01:15:04] .................................................................................................... 4900/5472
[01:15:09] .................................................................................................... 5000/5472
[01:15:12] .................................F..F............................................................... 5100/5472
[01:15:19] .................................................................................................... 5300/5472
[01:15:22] .................................................................................................... 5400/5472
[01:15:25] ..........i.............................................................
[01:15:25] failures:
[01:15:25] failures:
[01:15:25] 
[01:15:25] ---- [ui] ui/associated-types/associated-types-overridden-binding-2.rs stdout ----
[01:15:25] 
[01:15:25] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:15:25] status: exit code: 101
[01:15:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/auxiliary" "-A" "unused"
[01:15:25] ------------------------------------------
[01:15:25] 
[01:15:25] ------------------------------------------
[01:15:25] stderr:
[01:15:25] stderr:
[01:15:25] ------------------------------------------
[01:15:25] error: internal compiler error: src/librustc/hir/map/mod.rs:632: couldn't find hir id HirId { owner: DefIndex(0:0), local_id: 0 } in the HIR map
[01:15:25] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
[01:15:25] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:25] error: aborting due to previous error
[01:15:25] 
[01:15:25] 
[01:15:25] 
[01:15:25] note: the compiler unexpectedly panicked. this is a bug.
[01:15:25] 
[01:15:25] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:15:25] 
[01:15:25] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:15:25] 
[01:15:25] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:15:25] 
[01:15:25] ------------------------------------------
[01:15:25] 
[01:15:25] 
[01:15:25] 
[01:15:25] ---- [ui] ui/traits/trait-alias-ambiguous.rs stdout ----
[01:15:25] 
[01:15:25] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:15:25] status: exit code: 101
[01:15:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/auxiliary" "-A" "unused"
[01:15:25] ------------------------------------------
[01:15:25] 
[01:15:25] ------------------------------------------
[01:15:25] stderr:
[01:15:25] stderr:
[01:15:25] ------------------------------------------
[01:15:25] error: internal compiler error: src/librustc/hir/map/mod.rs:632: couldn't find hir id HirId { owner: DefIndex(0:0), local_id: 0 } in the HIR map
[01:15:25] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
[01:15:25] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:25] error: aborting due to previous error
[01:15:25] 
[01:15:25] 
[01:15:25] 
[01:15:25] note: the compiler unexpectedly panicked. this is a bug.
[01:15:25] 
[01:15:25] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:15:25] 
[01:15:25] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:15:25] 
[01:15:25] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:15:25] 
[01:15:25] ------------------------------------------
[01:15:25] 
[01:15:25] 
[01:15:25] 
[01:15:25] ---- [ui] ui/traits/trait-alias-object.rs stdout ---et _: &dyn IteratorAlias = &vec![123].into_iter(); //~ ERROR must be specified
[01:15:25]    |             ^^^^^^^^^^^^^^^^^ associated type `Item` must be specified
[01:15:25] 
[01:15:25] error: internal compiler error: src/librustc/hir/map/mod.rs:632: couldn't find hir id HirId { owner: DefIndex(0:0), local_id: 0 } in the HIR map
[01:15:25] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
[01:15:25] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:25] error: aborting due to 3 previous errors
[01:15:25] 
---
[01:15:25] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:15:25] 
[01:15:25] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:15:25] 
[01:15:25] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:15:25] 
[01:15:25] ------------------------------------------
[01:15:25] 
[01:15:25] 
---
[01:15:25]     [ui] ui/traits/trait-alias-object.rs
[01:15:25] 
[01:15:25] test result: FAILED. 5448 passed; 3 failed; 21 ignored; 0 measured; 0 filtered out
[01:15:25] 
[01:15:25] thread 'main' pted unsuccessfully in 0:04:28
[01:15:25] Makefile:48: recipe for target 'check' failed
[01:15:25] make: *** [check] Error 1
