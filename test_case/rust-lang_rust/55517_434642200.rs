plain
travis_time:end:03ecaf1c:start=1540976906560202587,finish=1540976988799046499,duration=82238843912
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:55:23] .................................................................................................... 100/4989
[00:55:26] .................................................................................................... 200/4989
[00:55:29] ...........................................................................................ii....... 300/4989
[00:55:32] .........................................................................................iii........ 400/4989
[00:55:35] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4989
[00:55:43] .................................................................................................... 700/4989
[00:55:50] ...................................................................i...........i.................... 800/4989
[00:55:53] .....................................................................................iiiii.......... 900/4989
[00:55:57] .................................................................................................... 1000/4989
---
[00:56:35] .................................................................................................... 2200/4989
[00:56:40] .................................................................................................... 2300/4989
[00:56:44] .................................................................................................... 2400/4989
[00:56:48] .................................................................................................... 2500/4989
[00:56:51] .........................................................................iiiiiiiii.................. 2600/4989
[00:56:59] ........................ii.......................................................................... 2800/4989
[00:57:02] .................................................................................................... 2900/4989
[00:57:06] .................................................................................................... 3000/4989
[00:57:09] ...................i................................................................................ 3100/4989
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:19] 
[01:11:19] running 115 tests
[01:11:22] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:11:22] .i....iiii.....
[01:11:22] 
[01:11:22]  finished in 3.707
[01:11:22] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:37] 
[01:11:37] running 118 tests
[01:12:02] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:12:06] ......iii.i.....ii
[01:12:06] 
[01:12:06]  finished in 28.431
[01:12:06] travis_fold:end:test_debuginfo

---
[01:38:52]     Finished release [optimized] target(s) in 51.93s
[01:38:52]      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_driver-7cb9d261b36bafe1
[01:38:52] 
[01:38:52] running 11 tests
[01:38:52] ....F.F....
[01:38:52] 
[01:38:52] ---- test::sub_free_bound_false_infer stdout ----
[01:38:52] ---- test::sub_free_bound_false_infer stdout ----
[01:38:52] thread 'test::sub_free_bound_false_infer' panicked at 'unexpected success computing sub(fn(_#0t) -> isize,for<'r> fn(&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) isize) -> isize)', librustc_driver/test.rs:452:17
[01:38:52] 
[01:38:52] ---- test::sub_free_bound_false stdout ----
[01:38:52] ---- test::sub_free_bound_false stdout ----
[01:38:52] thread 'test::sub_free_bound_false' panicked at 'unexpected success computing sub(fn(&ReFree(DefId(0/0:0 ~ test_crate[317d]), BrAnon(1)) isize) -> isize,for<'r> fn(&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) isize) -> isize)', librustc_driver/test.rs:452:17
[01:38:52] 
[01:38:52] failures:
[01:38:52]     test::sub_free_bound_false
[01:38:52]     test::sub_free_bound_false_infer
[01:38:52]     test::sub_free_bound_false_infer
[01:38:52] 
[01:38:52] test result: FAILED. 9 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:38:52] 
[01:38:52] error: test failed, to rerun pass '--lib'
[01:38:52] 
[01:38:52] 
[01:38:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:38:52] 
[01:38:52] 
[01:38:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:52] Build completed unsuccessfully in 0:47:30
[01:38:52] Build completed unsuccessfully in 0:47:30
[01:38:52] Makefile:58: recipe for target 'check' failed
[01:38:52] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03bbf8ea
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03f80e32:start=1540982942488725192,finish=1540982942493143780,duration=4418588
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07bd9308
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then prin
