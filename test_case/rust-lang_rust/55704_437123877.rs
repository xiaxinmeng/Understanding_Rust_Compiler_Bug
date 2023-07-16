plain
travis_time:end:14ce8bca:start=1541698916665810205,finish=1541698988581294421,duration=71915484216
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:09] .................................................................................................... 100/5000
[00:50:12] .................................................................................................... 200/5000
[00:50:14] ........................................................................ii...................ii..... 300/5000
[00:50:17] ...........................................................................................iii...... 400/5000
[00:50:20] ..iiiiiiii.iii...........................iii...........................................i...........i 500/5000
[00:50:27] .................................................................................................... 700/5000
[00:50:33] .....................................................................i...........i.................. 800/5000
[00:50:36] ........................................................................................iiiii....... 900/5000
[00:50:39] ............iiiiii.................................................................................. 1000/5000
---
[00:51:14] .................................................................................................... 2200/5000
[00:51:19] .................................................................................................... 2300/5000
[00:51:22] .................................................................................................... 2400/5000
[00:51:26] .................................................................................................... 2500/5000
[00:51:29] ....................................................................iiiiiiiii....................... 2600/5000
[00:51:36] ................................ii.................................................................. 2800/5000
[00:51:39] .................................................................................................... 2900/5000
[00:51:43] .................................................................................................... 3000/5000
[00:51:46] ...........................i........................................................................ 3100/5000
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:10] 
[01:05:10] running 115 tests
[01:05:13] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:05:13] .i.....iiii....
[01:05:13] 
[01:05:13]  finished in 3.416
[01:05:13] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:28] 
[01:05:28] running 119 tests
[01:05:53] .iiiii...i.....i..i...i..i.i..i.i...i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i. 100/119
[01:05:57] i......iii.i.....ii
[01:05:57] 
[01:05:57]  finished in 29.050
[01:05:57] travis_fold:end:test_debuginfo

---
[01:39:34] doc tests for: /checkout/src/doc/unstable-book/src/language-features/const-fn.md
[01:39:35] doc tests for: /checkout/src/doc/unstable-book/src/language-features/crate-visibility-modifier.md
[01:39:35] doc tests for: /checkout/src/doc/unstable-book/src/language-features/custom-test-frameworks.md
[01:39:35] doc tests for: /checkout/src/doc/unstable-book/src/language-features/doc-alias.md
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected normal fn, found unsafe fn
[01:39:36]    |
[01:39:36]    = note: expected type `fn(std::pin::Pin<&mut main::__Generator>) -> std::ops::GeneratorState<i32, &'static str>`
[01:39:36]               found type `unsafe fn(&mut main::__Generator) -> std::ops::GeneratorState<i32, &'static str>`
[01:39:36] thread '/checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::Generators_as_state_machines (line 185)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:39:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:39:36] 
[01:39:36] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators (line 59) stdout ----
[01:39:36] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators (line 59) stdout ----
[01:39:36] error[E0599]: no method named `resume` found for type `[generator@/checkout/src/doc/unstable-book/src/language-features/generators.md:7:25: 11:6 _]` in the current scope
[01:39:36]    |
[01:39:36]    |
[01:39:36] 14 |     unsafe { generator.resume() };
[01:39:36] 
[01:39:36] 
[01:39:36] error[E0599]: no method named `resume` found for type `[generator@/checkout/src/doc/unstable-book/src/language-features/generators.md:7:25: 11:6 _]` in the current scope
[01:39:36]    |
[01:39:36]    |
[01:39:36] 16 |     unsafe { generator.resume() };
[01:39:36] 
[01:39:36] thread '/checkout/src/doc/unstable-book/src/language-features/generators.md - generators (line 59)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:39:36] 
[01:39:36] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::Generators_as_state_machines (line 166) stdout ----
[01:39:36] ---- /checkout/src/doc/unstable-book/src/language-features/generators.md - generators::_::Generators_as_state_machines (line 166) stdout ----
[01:39:36] error[E0599]: no method named `resume` found for type `[generator@/checkout/src/doc/unstable-book/src/language-features/generators.md:8:25: 11:6 ret:_ _]` in the current scope
[01:39:36]    |
[01:39:36]    |
[01:39:36] 13 |     unsafe { generator.resume() };
[01:39:36] 
[01:39:36] 
[01:39:36] error[E0599]: no method named `resume` found for type `[generator@/checkout/src/doc/unstable-book/src/language-features/generators.md:8:25: 11:6 ret:_ _]` in the current scope
[01:39:36]    |
[01:39:36]    |
[01:39:36] 14 |     unsafe { generator.resume() };
[01:39:36] 
[01:39:36] 
[01:39:36] error[E0698]: type inside generator must be known in this context
[01:39:36]   |
[01:39:36] 9 |         yield 1;
[01:39:36]   |               ^
[01:39:36]   |
[01:39:36]   |
[01:39:36] note: the type is part of the generator because of this `yield`
[01:39:36]   |
[01:39:36] 9 |         yield 1;
[01:39:36]   |         ^^^^^^^
[01:39:36] 
---
[01:39:36] 
[01:39:36] 
[01:39:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:36] Build completed unsuccessfully in 0:53:09
[01:39:36] Makefile:58: recipe for target 'check' failed
[01:39:36] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02b0a750
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03da6a5a:start=1541704977904294335,finish=1541704977910144702,duration=5850367
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1314644c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1
