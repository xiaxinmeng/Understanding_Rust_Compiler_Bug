plain
travis_time:end:1d144326:start=1554404558230062618,finish=1554404646190781006,duration=87960718388
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:52] 
[01:13:52] running 9 tests
[01:13:52] iiiiiiiii
[01:13:52] 
[01:13:52]  finished in 0.153
[01:13:52] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:08] 
[01:14:08] running 121 tests
[01:14:33] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:14:37] i.i......iii.i.....ii
[01:14:37] 
[01:14:37]  finished in 29.710
[01:14:37] travis_fold:end:test_debuginfo

---
[01:25:51] 
[01:25:51]    Doc-tests core
[01:25:56] 
[01:25:56] running 2308 tests
[01:26:09] ....iiiii........................................................................................... 100/2308
[01:26:22] ......................................................................ii.....F...................... 200/2308
[01:26:52] .................................................................................................... 400/2308
[01:27:04] .......................i..i......................................................................... 500/2308
[01:27:17] .................................................................................................... 600/2308
[01:27:30] .................................................................................................... 700/2308
---
[01:31:09] ---- convert.rs - convert::TryFrom (line 434) stdout ----
[01:31:09] error[E0405]: cannot find trait `TryFrom` in this scope
[01:31:09]  --> convert.rs:437:6
[01:31:09]   |
[01:31:09] 6 | impl TryFrom<i32> for SuperiorThanZero {
[01:31:09] help: possible candidate is found in another module, you can import it into scope
[01:31:09]   |
[01:31:09] 3 | use std::convert::TryFrom;
[01:31:09]   |
---
[01:31:09] 
[01:31:09] error: test failed, to rerun pass '--doc'
[01:31:09] 
[01:31:09] 
[01:31:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:31:09] 
[01:31:09] 
[01:31:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:09] Build completed unsuccessfully in 0:29:11
[01:31:09] Build completed unsuccessfully in 0:29:11
[01:31:09] Makefile:48: recipe for target 'check' failed
[01:31:09] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:252ab738
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr  4 20:35:25 UTC 2019
---
travis_time:end:000cc741:start=1554410126743621859,finish=1554410126748968759,duration=5346900
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e3186e8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:015de621
travis_time:start:015de621
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0afc8af0
$ dmesg | grep -i kill
