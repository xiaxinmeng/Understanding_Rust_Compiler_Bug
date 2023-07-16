plain
travis_time:end:1d18ff20:start=1543249633064657167,finish=1543249687328807110,duration=54264149943
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:45:36] .................................................................................................... 100/5067
[00:45:38] .................................................................................................... 200/5067
[00:45:41] .............................ii............................................ii...................ii.. 300/5067
[00:45:43] ..............................................................................................iii... 400/5067
[00:45:46] .....iiiiiiii.iii............................iii...........................................i........ 500/5067
[00:45:52] .................................................................................................... 700/5067
[00:45:57] ................................................................................................i... 800/5067
[00:46:01] ........i........................................................................................... 900/5067
[00:46:04] ...............iiiii..................ii.iiii....................................................... 1000/5067
[00:46:04] ...............iiiii..................ii.iiii....................................................... 1000/5067
[00:46:06] ..........................................................................................iiiiiiii.. 1100/5067
[00:46:10] .................................................................................................... 1300/5067
[00:46:12] .................................................................................................... 1400/5067
[00:46:14] .............................................i...................................................... 1500/5067
[00:46:17] ..............i.........ii.........................................................i................ 1600/5067
---
[00:46:40] .................................................................................................... 2300/5067
[00:46:43] .................................................................................................... 2400/5067
[00:46:47] .................................................................................................... 2500/5067
[00:46:50] .................................................................................................... 2600/5067
[00:46:53] ...iiiiiiiii........................................................................................ 2700/5067
[00:46:58] .................................................................................................... 2900/5067
[00:47:02] .................................................................................................... 3000/5067
[00:47:04] ..................................................................i................................. 3100/5067
[00:47:06] .................................................................................................... 3200/5067
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:45] 
[00:59:45] running 117 tests
[00:59:48] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[00:59:48] i.i.....iiii.....
[00:59:48] 
[00:59:48]  finished in 3.122
[00:59:48] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:01] 
[01:00:01] running 118 tests
[01:00:23] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:00:26] ......iii.i.....ii
[01:00:26] 
[01:00:26]  finished in 24.996
[01:00:26] travis_fold:end:test_debuginfo

---
[01:09:08]    Compiling test v0.0.0 (/checkout/src/libtest)
[01:09:08] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:09:08]     --> src/libtest/lib.rs:1801:9
[01:09:08]      |
[01:09:08] 1393 | / pub fn run_test(
[01:09:08] 1394 | |     opts: &TestOpts,
[01:09:08] 1395 | |     force_ignore: bool,
[01:09:08] 1396 | |     test: TestDescAndFn,
[01:09:08] 1480 | |     }
[01:09:08] 1481 | | }
[01:09:08]      | |_- defined here
[01:09:08] ...
[01:09:08] ...
[01:09:08] 1801 |           run_test(&TestOpts::new(), false, desc, tx);
[01:09:08] 
[01:09:08] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:09:08]     --> src/libtest/lib.rs:1819:9
[01:09:08]      |
[01:09:08]      |
[01:09:08] 1393 | / pub fn run_test(
[01:09:08] 1394 | |     opts: &TestOpts,
[01:09:08] 1395 | |     force_ignore: bool,
[01:09:08] 1396 | |     test: TestDescAndFn,
[01:09:08] 1480 | |     }
[01:09:08] 1481 | | }
[01:09:08]      | |_- defined here
[01:09:08] ...
[01:09:08] ...
[01:09:08] 1819 |           run_test(&TestOpts::new(), false, desc, tx);
[01:09:08] 
[01:09:08] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:09:08]     --> src/libtest/lib.rs:1839:9
[01:09:08]      |
[01:09:08]      |
[01:09:08] 1393 | / pub fn run_test(
[01:09:08] 1394 | |     opts: &TestOpts,
[01:09:08] 1395 | |     force_ignore: bool,
[01:09:08] 1396 | |     test: TestDescAndFn,
[01:09:08] 1480 | |     }
[01:09:08] 1481 | | }
[01:09:08]      | |_- defined here
[01:09:08] ...
[01:09:08] ...
[01:09:08] 1839 |           run_test(&TestOpts::new(), false, desc, tx);
[01:09:08] 
[01:09:08] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:09:08]     --> src/libtest/lib.rs:1859:9
[01:09:08]      |
[01:09:08]      |
[01:09:08] 1393 | / pub fn run_test(
[01:09:08] 1394 | |     opts: &TestOpts,
[01:09:08] 1395 | |     force_ignore: bool,
[01:09:08] 1396 | |     test: TestDescAndFn,
[01:09:08] 1480 | |     }
[01:09:08] 1481 | | }
[01:09:08]      | |_- defined here
[01:09:08] ...
[01:09:08] ...
[01:09:08] 1859 |           run_test(&TestOpts::new(), false, desc, tx);
[01:09:08] 
[01:09:08] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:09:08]     --> src/libtest/lib.rs:1881:9
[01:09:08]      |
[01:09:08]      |
[01:09:08] 1393 | / pub fn run_test(
[01:09:08] 1394 | |     opts: &TestOpts,
[01:09:08] 1395 | |     force_ignore: bool,
[01:09:08] 1396 | |     test: TestDescAndFn,
[01:09:08] 1480 | |     }
[01:09:08] 1481 | | }
[01:09:08]      | |_- defined here
[01:09:08] ...
[01:09:08] ...
[01:09:08] 1881 |           run_test(&TestOpts::new(), false, desc, tx);
[01:09:08] 
[01:09:08] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:09:08]     --> src/libtest/lib.rs:1899:9
[01:09:08]      |
[01:09:08]      |
[01:09:08] 1393 | / pub fn run_test(
[01:09:08] 1394 | |     opts: &TestOpts,
[01:09:08] 1395 | |     force_ignore: bool,
[01:09:08] 1396 | |     test: TestDescAndFn,
[01:09:08] 1480 | |     }
[01:09:08] 1481 | | }
[01:09:08]      | |_- defined here
[01:09:08] ...
[01:09:08] ...
[01:09:08] 1899 |           run_test(&TestOpts::new(), false, desc, tx);
[01:09:08] 
[01:09:09] error: aborting due to 6 previous errors
[01:09:09] 
[01:09:09] For more information about this error, try `rustc --explain E0061`.
[01:09:09] For more information about this error, try `rustc --explain E0061`.
[01:09:09] error: Could not compile `test`.
[01:09:09] 
[01:09:09] To learn more, run the command again with --verbose.
[01:09:09] 
[01:09:09] 
[01:09:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "test" "--" "--quiet"
[01:09:09] 
[01:09:09] 
[01:09:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:09] Build completed unsuccessfully in 0:27:08
[01:09:09] Build completed unsuccessfully in 0:27:08
[01:09:09] Makefile:58: recipe for target 'check' failed
[01:09:09] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0771ceb8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 26 17:37:24 UTC 2018
---
travis_time:end:0081310c:start=1543253847070341510,finish=1543253847074430187,duration=4088677
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05cd2235
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\
