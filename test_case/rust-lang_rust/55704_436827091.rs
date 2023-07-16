plain
travis_time:end:17ab3d7e:start=1541630891639197812,finish=1541630946689239275,duration=55050041463
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:01] .................................................................................................... 100/5000
[00:48:03] .................................................................................................... 200/5000
[00:48:06] ........................................................................ii...................ii..... 300/5000
[00:48:09] ...........................................................................................iii...... 400/5000
[00:48:11] ..iiiiiiii.iii...........................iii...........................................i...........i 500/5000
[00:48:18] .................................................................................................... 700/5000
[00:48:24] .....................................................................i...........i.................. 800/5000
[00:48:27] ........................................................................................iiiii....... 900/5000
[00:48:30] ...........ii.iiii.................................................................................. 1000/5000
---
[00:49:04] .................................................................................................... 2200/5000
[00:49:08] .................................................................................................... 2300/5000
[00:49:12] .................................................................................................... 2400/5000
[00:49:15] .................................................................................................... 2500/5000
[00:49:18] ....................................................................iiiiiiiii....................... 2600/5000
[00:49:25] ................................ii.................................................................. 2800/5000
[00:49:27] .................................................................................................... 2900/5000
[00:49:31] .................................................................................................... 3000/5000
[00:49:33] ...........................i........................................................................ 3100/5000
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:25] 
[01:01:25] running 115 tests
[01:01:28] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:01:29] .i....iiii.....
[01:01:29] 
[01:01:29]  finished in 3.308
[01:01:29] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:42] 
[01:01:42] running 119 tests
[01:02:06] .iiiii...i.....i..i...i..i.i..i.i...i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i. 100/119
[01:02:09] i......iii.i.....ii
[01:02:09] 
[01:02:09]  finished in 27.163
[01:02:09] travis_fold:end:test_debuginfo

---
[01:32:47] 
[01:32:47] failures:
[01:32:47] 
[01:32:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10741) stdout ----
[01:32:47] error[E0599]: no method named `resume` found for type `[generator@/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5:13: 9:2 _]` in the current scope
[01:32:47]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10749:12
[01:32:47]    |
[01:32:47] 10 | unsafe { b.resume() };
[01:32:47] 
[01:32:47] 
[01:32:47] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10741)' panicked at 'Some expected error codes were not found: ["E0626"]', librustdoc/test.rs:328:9
[01:32:47] 
[01:32:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10759) stdout ----
[01:32:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10759) stdout ----
[01:32:47] error[E0599]: no method named `resume` found for type `[generator@/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5:13: 9:2 _]` in the current scope
[01:32:47]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10767:12
[01:32:47]    |
[01:32:47] 10 | unsafe { b.resume() };
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10763:9
[01:32:47] 6 |     let a = 3;
[01:32:47]   |         ^
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10764:5
[01:32:47]   |
[01:32:47] 7 |     yield ();
[01:32:47] 
[01:32:47] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10759)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:32:47] 
[01:32:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10776) stdout ----
[01:32:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10776) stdout ----
[01:32:47] error[E0599]: no method named `resume` found for type `[generator@/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5:13: 10:2 _]` in the current scope
[01:32:47]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10785:12
[01:32:47]    |
[01:32:47] 11 | unsafe { b.resume() };
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10780:7
[01:32:47]   |
[01:32:47] 6 |   let v = vec![1,2,3];
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10782:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // ...when this yield occurs.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10781:13
[01:32:47]   |
[01:32:47] 7 |   for &x in &v { // <-- borrow of `v` is still in scope...
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10782:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // ...when this yield occurs.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10781:14
[01:32:47]   |
[01:32:47] 7 |   for &x in &v { // <-- borrow of `v` is still in scope...
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10782:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // ...when this yield occurs.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10781:13
[01:32:47]   |
[01:32:47] 7 |   for &x in &v { // <-- borrow of `v` is still in scope...
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10782:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // ...when this yield occurs.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10781:8
[01:32:47]   |
[01:32:47] 7 |   for &x in &v { // <-- borrow of `v` is still in scope...
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10782:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // ...when this yield occurs.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10782:11
[01:32:47]   |
[01:32:47] 8 |     yield x; // ...when this yield occurs.
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10782:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // ...when this yield occurs.
[01:32:47] 
[01:32:47] 
[01:32:47] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10776)' panicked at 'Some expected error codes were not found: ["E0626"]', librustdoc/test.rs:328:9
[01:32:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10791) stdout ----
[01:32:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10791) stdout ----
[01:32:47] error[E0599]: no method named `resume` found for type `[generator@/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5:13: 10:2 _]` in the current scope
[01:32:47]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10800:12
[01:32:47]    |
[01:32:47] 11 | unsafe { b.resume() };
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10795:7
[01:32:47]   |
[01:32:47] 6 |   let v = vec![1,2,3];
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10797:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // <-- Now yield is OK.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10796:12
[01:32:47]   |
[01:32:47] 7 |   for x in v { // <-- Take ownership of the values instead!
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10797:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // <-- Now yield is OK.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10796:12
[01:32:47]   |
[01:32:47] 7 |   for x in v { // <-- Take ownership of the values instead!
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10797:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // <-- Now yield is OK.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10796:7
[01:32:47]   |
[01:32:47] 7 |   for x in v { // <-- Take ownership of the values instead!
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10797:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // <-- Now yield is OK.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10797:11
[01:32:47]   |
[01:32:47] 8 |     yield x; // <-- Now yield is OK.
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10797:5
[01:32:47]   |
[01:32:47] 8 |     yield x; // <-- Now yield is OK.
[01:32:47] 
[01:32:47] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10791)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:32:47] 
[01:32:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10805) stdout ----
[01:32:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10805) stdout ----
[01:32:47] error[E0599]: no method named `resume` found for type `[generator@/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5:13: 12:2 _]` in the current scope
[01:32:47]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10816:12
[01:32:47]    |
[01:32:47] 13 | unsafe { b.resume() };
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10809:7
[01:32:47]   |
[01:32:47] 6 |   let v = vec![1,2,3];
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10813:5
[01:32:47]   |
[01:32:47] 10|     yield x; // <-- Now yield is OK.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10812:9
[01:32:47]   |
[01:32:47] 9 |     let x = v[i]; // (*)
[01:32:47]   |
[01:32:47]   |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10813:5
[01:32:47]   |
[01:32:47] 10|     yield x; // <-- Now yield is OK.
[01:32:47] 
[01:32:47] 
[01:32:47] error[E0698]: type inside generator must be known in this context
[01:32:47]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10813:11
[01:32:47]    |
[01:32:47] 10 |     yield x; // <-- Now yield is OK.
[01:32:47]    |
[01:32:47]    |
[01:32:47] note: the type is part of the generator because of this `yield`
[01:32:47]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10813:5
[01:32:47]    |
[01:32:47] 10 |     yield x; // <-- Now yield is OK.
[01:32:47] 
[01:32:47] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 10805)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:32:47] 
[01:32:47] 
---
[01:32:47] 
[01:32:47] 
[01:32:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:47] Build completed unsuccessfully in 0:48:18
[01:32:47] make: *** [check] Error 1
[01:32:47] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:34c7013c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:17097852:start=1541636526678707974,finish=1541636526683975158,duration=5267184
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:222e35ec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:344802da
travis_time:start:344802da
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00ee78e8
$ dmesg | grep -i kill
