plain
travis_time:end:038c1719:start=1550082648400920453,finish=1550082822735458805,duration=174334538352
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:56] 
[01:13:56] running 119 tests
[01:14:21] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:14:25] i......iii.i.....ii
[01:14:25] 
[01:14:25]  finished in 28.738
[01:14:25] travis_fold:end:test_debuginfo

---
[01:24:55] 
[01:24:55]    Doc-tests core
[01:25:00] 
[01:25:00] running 2253 tests
[01:25:11] ....i..iiiiii....................................................................................... 100/2253
[01:25:22] .............................................................................F...................... 200/2253
[01:25:50] .................................................................................................... 400/2253
[01:26:02] .................................................................................................... 500/2253
[01:26:13] .................................................................................................... 600/2253
[01:26:25] .................................................................................................... 700/2253
---
[01:29:40] ---- convert.rs - convert::TryFrom (line 414) stdout ----
[01:29:40] error[E0599]: no function or associated item named `try_from` found for type `i32` in the current scope
[01:29:40]   --> convert.rs:423:31
[01:29:40]    |
[01:29:40] 12 | let try_smaller_number = i32::try_from(big_number);
[01:29:40]    |                          |
[01:29:40]    |                          function or associated item not found in `i32`
[01:29:40]    |
[01:29:40]    = help: items from traits can only be used if the trait is in scope
[01:29:40]    = help: items from traits can only be used if the trait is in scope
[01:29:40] help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:29:40]    |
[01:29:40] 3  | use std::convert::TryFrom;
[01:29:40] 
[01:29:40] error[E0599]: no function or associated item named `try_from` found for type `i32` in the current scope
[01:29:40]   --> convert.rs:427:42
[01:29:40]    |
[01:29:40]    |
[01:29:40] 16 | let try_successful_smaller_number = i32::try_from(3);
[01:29:40]    |                                     |
[01:29:40]    |                                     function or associated item not found in `i32`
[01:29:40]    |
[01:29:40]    = help: items from traits can only be used if the trait is in scope
[01:29:40]    = help: items from traits can only be used if the trait is in scope
[01:29:40] help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:29:40]    |
[01:29:40] 3  | use std::convert::TryFrom;
[01:29:40] 
[01:29:40] 
[01:29:40] thread 'convert.rs - convert::TryFrom (line 414)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:29:40] 
[01:29:40] 
[01:29:40] failures:
[01:29:40]     convert.rs - convert::TryFrom (line 414)
[01:29:40]     convert.rs - convert::TryFrom (line 414)
[01:29:40] 
[01:29:40] test result: FAILED. 2242 passed; 1 failed; 10 ignored; 0 measured; 0 filtered out
[01:29:40] 
[01:29:40] error: test failed, to rerun pass '--doc'
[01:29:40] 
[01:29:40] 
[01:29:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:29:40] 
[01:29:40] 
[01:29:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:40] Build completed unsuccessfully in 0:27:02
[01:29:40] Build completed unsuccessfully in 0:27:02
[01:29:40] Makefile:48: recipe for target 'check' failed
[01:29:40] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01e1bf4e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 20:03:32 UTC 2019
---
travis_time:end:0302c300:start=1550088214066318358,finish=1550088214070848906,duration=4530548
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05b62ef8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ab13b29
travis_time:start:0ab13b29
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bddaf7f
$ dmesg | grep -i kill
