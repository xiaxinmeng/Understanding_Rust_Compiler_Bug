plain
[00:50:26] .................................................................................................... 2200/4932
[00:50:30] ...................................i................................................................ 2300/4932
[00:50:34] .................................................................................................... 2400/4932
[00:50:38] .................................................................................................... 2500/4932
[00:50:41] ..................................................iiiiiiiii......................................... 2600/4932
[00:50:48] .................................................................................................... 2800/4932
[00:50:52] .................................................................................................... 2900/4932
[00:50:55] ................................................................................i................... 3000/4932
[00:50:58] .................................................................................................... 3100/4932
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:50] 
[01:03:50] running 111 tests
[01:03:54] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:03:54] ..iiii.....
[01:03:54] 
[01:03:54]  finished in 3.471
[01:03:54] travis_fold:end:test_codegen

---
[01:38:29] 
[01:38:29] failures:
[01:38:29] 
[01:38:29] ---- /checkout/src/doc/unstable-book/src/language-features/trait-alias.md - trait_alias (line 14) stdout ----
[01:38:29] error: expected one of `(`, `+`, `::`, `;`, `<`, or `where`, found `:`
[01:38:29]   |
[01:38:29]   |
[01:38:29] 4 | trait Foo = std:fmt::Debug + Send;
[01:38:29]   |                ^ expected one of `(`, `+`, `::`, `;`, `<`, or `where` here
[01:38:29] thread '/checkout/src/doc/unstable-book/src/language-features/trait-alias.md - trait_alias (line 14)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:38:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:38:29] 
[01:38:29] 
---
[01:38:29] 
[01:38:29] 
[01:38:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:29] Build completed unsuccessfully in 0:52:47
[01:38:29] Makefile:58: recipe for target 'check' failed
[01:38:29] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a3fc9db
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:051ea7aa:start=1540258177642529276,finish=1540258177649475831,duration=6946555
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13e49cee
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14971fb8
travis_time:start:14971fb8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08d1f0ac
$ dmesg | grep -i kill
