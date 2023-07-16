plain
travis_time:end:2ebcf74a:start=1543615917310230679,finish=1543615918332185156,duration=1021954477
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:19] .................................................................................................... 100/5104
[00:47:21] .................................................................................................... 200/5104
[00:47:24] .............................ii............................................ii...................ii.. 300/5104
[00:47:26] ..............................................................................................iii... 400/5104
[00:47:29] .....iiiiiiii.iii............................iii...........................................i........ 500/5104
[00:47:35] .................................................................................................... 700/5104
[00:47:41] .................................................................................................i.. 800/5104
[00:47:45] .............i...................................................................................... 900/5104
[00:47:48] ....................iiiii..................ii.iiii.................................................. 1000/5104
---
[00:48:24] .................................................................................................... 2300/5104
[00:48:28] .................................................................................................... 2400/5104
[00:48:32] .................................................................................................... 2500/5104
[00:48:35] .................................................................................................... 2600/5104
[00:48:38] ............iiiiiiiii............................................................................... 2700/5104
[00:48:44] .................................................................................................... 2900/5104
[00:48:48] .................................................................................................... 3000/5104
[00:48:51] ..........................................................................i......................... 3100/5104
[00:48:53] .................................................................................................... 3200/5104
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:07] 
[01:02:07] running 119 tests
[01:02:09] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[01:02:10] .ii.i.....iiii.....
[01:02:10] 
[01:02:10]  finished in 3.238
[01:02:10] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:24] 
[01:02:24] running 118 tests
[01:02:46] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:02:50] ......iii.i.....ii
[01:02:50] 
[01:02:50]  finished in 26.276
[01:02:50] travis_fold:end:test_debuginfo

---
[01:30:14] 
[01:30:14] failures:
[01:30:14] 
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::Generators_as_state_machines (line 166) stdout ----
[01:30:14] error: failed to acquire jobserver token: Bad file descriptor (os error 9)
[01:30:14] thread '/checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::Generators_as_state_machines (line 166)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:30:14] 
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators (line 59) stdout ----
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators (line 59) stdout ----
[01:30:14] error: failed to acquire jobserver token: Bad file descriptor (os error 9)
[01:30:14] thread '/checkout/src/doc/unstable-book/src/language-features/generators.md - generators (line 59)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:30:14] 
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::Generators_as_state_machines (line 185) stdout ----
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::Generators_as_state_machines (line 185) stdout ----
[01:30:14] error: failed to acquire jobserver token: Bad file descriptor (os error 9)
[01:30:14] thread '/checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::Generators_as_state_machines (line 185)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:30:14] 
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators (line 28) stdout ----
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators (line 28) stdout ----
[01:30:14] error: failed to acquire jobserver token: Bad file descriptor (os error 9)
[01:30:14] thread '/checkout/src/doc/unstable-book/src/language-features/generators.md - generators (line 28)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:30:14] 
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::The_ (line 107) stdout ----
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::The_ (line 107) stdout ----
[01:30:14] error: failed to acquire jobserver token: Bad file descriptor (os error 9)
[01:30:14] thread '/checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::The_ (line 107)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:30:14] 
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::The_ (line 88) stdout ----
[01:30:14] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::The_ (line 88) stdout ----
[01:30:14] error: failed to acquire jobserver token: Bad file descriptor (os error 9)
[01:30:14] thread '/checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::The_ (line 88)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:30:14] 
[01:30:14] 
[01:30:14] failures:
---
[01:30:14] 
[01:30:14] test result: FAILED. 0 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out
[01:30:14] 
[01:30:14] 
[01:30:14] stderr ----
[01:30:14] thread '<unnamed>' panicked at 'failed to acquire jobserver token: Bad file descriptor (os error 9)', src/librustc_codegen_ssa/back/write.rs:1310:29
[01:30:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:30:14] thread '<unnamed>' panicked at 'failed to acquire jobserver token: Bad file descriptor (os error 9)', src/librustc_codegen_ssa/back/write.rs:1310:29
[01:30:14] thread '<unnamed>' panicked at 'failed to acquire jobserver token: Bad file descriptor (os error 9)', src/librustc_codegen_ssa/back/write.rs:1310:29
[01:30:14] thread '<unnamed>' panicked at 'failed to acquire jobserver token: Bad file descriptor (os error 9)', src/librustc_codegen_ssa/back/write.rs:1310:29
[01:30:14] thread 'thread '<unnamed><unnamed>' panicked at '' panicked at 'failed to acquire jobserver token: Bad file descriptor (os error 9)failed to acquire jobserver token: Bad file descriptor (os error 9)', ', src/librustc_codegen_ssa/back/write.rssrc/librustc_codegen_ssa/back/write.rs::13101310::2929
[01:30:14] 
[01:30:14] 
[01:30:14] 
[01:30:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:14] Build completed unsuccessfully in 0:46:40
[01:30:14] Makefile:58: recipe for target 'check' failed
[01:30:14] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:046128c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 30 23:42:21 UTC 2018
---
travis_time:end:0de14ce4:start=1543621345204264429,finish=1543621345208147474,duration=3883045
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0163b67b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a26d77c
travis_time:start:0a26d77c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0316687e
$ dmesg | grep -i kill
