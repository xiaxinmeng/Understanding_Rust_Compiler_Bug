plain
travis_time:end:002c22c0:start=1540933444313701495,finish=1540933521747995975,duration=77434294480
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:53:38] .................................................................................................... 100/4989
[00:53:41] .................................................................................................... 200/4989
[00:53:45] ...........................................................................................ii....... 300/4989
[00:53:48] .........................................................................................iii........ 400/4989
[00:53:50] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4989
[00:53:58] .................................................................................................... 700/4989
[00:54:05] ...................................................................i...........i.................... 800/4989
[00:54:07] .....................................................................................iiiii.......... 900/4989
[00:54:11] .................................................................................................... 1000/4989
---
[00:54:48] .................................................................................................... 2200/4989
[00:54:52] .................................................................................................... 2300/4989
[00:54:56] .................................................................................................... 2400/4989
[00:55:00] .................................................................................................... 2500/4989
[00:55:04] .........................................................................iiiiiiiii.................. 2600/4989
[00:55:11] ........................ii.......................................................................... 2800/4989
[00:55:14] .................................................................................................... 2900/4989
[00:55:18] .................................................................................................... 3000/4989
[00:55:20] ...................i................................................................................ 3100/4989
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:50] 
[01:08:50] running 112 tests
[01:08:54] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii.i. 100/112
[01:08:54] test result: ok. 82 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:08:54] 
[01:08:54]  finished in 3.589
[01:08:54] travis_fold:end:test_codegen
---
[01:35:58]     Finished release [optimized] target(s) in 52.28s
[01:35:58]      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_driver-7cb9d261b36bafe1
[01:35:58] 
[01:35:58] running 11 tests
[01:35:58] ...F.F.....
[01:35:58] 
[01:35:58] ---- test::sub_free_bound_false stdout ----
[01:35:58] ---- test::sub_free_bound_false stdout ----
[01:35:58] thread 'test::sub_free_bound_false' panicked at 'unexpected success computing sub(fn(&ReFree(DefId(0/0:0 ~ test_crate[317d]), BrAnon(1)) isize) -> isize,for<'r> fn(&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) isize) -> isize)', librustc_driver/test.rs:452:17
[01:35:58] 
[01:35:58] ---- test::sub_free_bound_false_infer stdout ----
[01:35:58] ---- test::sub_free_bound_false_infer stdout ----
[01:35:58] thread 'test::sub_free_bound_false_infer' panicked at 'unexpected success computing sub(fn(_#0t) -> isize,for<'r> fn(&ReLateBound(DebruijnIndex(0), BrNamed(crate0:DefIndex(0:0), 'r)) isize) -> isize)', librustc_driver/test.rs:452:17
[01:35:58] 
[01:35:58] failures:
[01:35:58]     test::sub_free_bound_false
[01:35:58]     test::sub_free_bound_false_infer
[01:35:58]     test::sub_free_bound_false_infer
[01:35:58] 
[01:35:58] test result: FAILED. 9 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:35:58] 
[01:35:58] error: test failed, to rerun pass '--lib'
[01:35:58] 
[01:35:58] 
[01:35:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:35:58] 
[01:35:58] 
[01:35:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:58] Build completed unsuccessfully in 0:46:15
[01:35:58] Build completed unsuccessfully in 0:46:15
[01:35:58] make: *** [check] Error 1
[01:35:58] Makefile:58: recipe for target 'check' failed
 travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0da64da3
travis_time:start:0da64da3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d1b5571
$ dmesg | grep -i kill
