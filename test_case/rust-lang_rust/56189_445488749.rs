plain
travis_time:end:2ae0cfc0:start=1544297028912724404,finish=1544297031029122736,duration=2116398332
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:24] 
[00:53:24] running 120 tests
[00:53:27] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:53:27] ..ii.i.....iiii.....
[00:53:27] 
[00:53:27]  finished in 3.364
[00:53:27] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:42] 
[00:53:42] running 118 tests
[00:54:05] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:54:08] ......iii.i.....ii
[00:54:08] 
[00:54:08]  finished in 27.050
[00:54:08] travis_fold:end:test_debuginfo

---
[01:23:48] 
[01:23:48] 12 3 | no
[01:23:48] 13   | ^^ not found in this scope
[01:23:48] 14 
[01:23:48] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:23:48] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:360:13
[01:23:48] 17 
[01:23:48] 17 
[01:23:48] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 21) stdout ----
[01:23:48] 
[01:23:48] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:23:48] 23 
[01:23:48] - ', src/librustdoc/test.rs:361:17
[01:23:48] + ', src/librustdoc/test.rs:395:17
[01:23:48] 25 
[01:23:48] 25 
[01:23:48] 26 
[01:23:48] 27 failures:
[01:23:48] 
[01:23:48] 
[01:23:48] The actual stdout differed from the expected stdout.
[01:23:48] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:23:48] To update references, rerun the tests and pass the `--bless` flag
[01:23:48] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:23:48] error: 1 errors occurred comparing output.
[01:23:48] error: 1 errors occurred comparing output.
[01:23:48] statusoc-ui/failed-doctest-output.rs - SomeStruct (line 21)' panicked at 'test executable failed:
[01:23:48] 
[01:23:48] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:23:48] 
[01:23:48] ', src/librustdoc/test.rs:395:17
[01:23:48] 
[01:23:48] 
[01:23:48] 
[01:23:48] failures:
[01:23:48]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27)
[01:23:48]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21)
[01:23:48] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:23:48] 
[01:23:48] 
[01:23:48] ------------------------------------------
---
[01:23:48] 
[01:23:48] 
[01:23:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-liafter_failure.4
travis_time:start:0decf013
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:046775b8
travis_time:start:046775b8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:055a487a
$ dmesg | grep -i kill
