plain
travis_time:end:03d4ab60:start=1553366511306987329,finish=1553366513728653727,duration=2421666398
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
[02:03:44] 
[02:03:44] running 9 tests
[02:03:44] iiiiiiiii
[02:03:44] 
[02:03:44]  finished in 0.171
[02:03:44] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[02:04:02] 
[02:04:02] running 120 tests
[02:04:33] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[02:04:39] .i......iii.i.....ii
[02:04:39] 
[02:04:39]  finished in 36.669
[02:04:39] travis_fold:end:test_debuginfo

---
[02:13:55] error: aborting due to previous error
[02:13:55] 
[02:13:55] 
[02:13:55] running 420 tests
[02:14:16] .........F.......................................................................................... 100/420
[02:14:34] .............................................................................i...................... 200/420
[02:14:50] .......................................................................F.F.......................... 300/420
[02:15:05] ..........................FF........................................................................ 400/420
[02:15:08] ....F...............
[02:15:08] 
[02:15:08] ---- boxed.rs - boxed (line 53) stdout ----
[02:15:08] thread 'rustc' panicked at 'value was not set', src/libcore/option.rs:1034:5
[02:15:08] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[02:15:08] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[02:15:08] thread 'boxed.rs - boxed (line 53)' panicked at 'Some expected error codes were not found: ["E0072"]', src/librustdoc/test.rs:313:9
[02:15:08] ---- string.rs - string::String (line 117) stdout ----
[02:15:08] thread 'rustc' panicked at 'value was not set', src/libcore/option.rs:1034:5
[02:15:08] thread 'rustc' panicked at 'value was not set', src/libcore/option.rs:1034:5
[02:15:08] thread 'string.rs - string::String (line 117)' panicked at 'Some expected error codes were not found: ["E0277"]', src/librustdoc/test.rs:313:9
[02:15:08] ---- string.rs - string::String (line 160) stdout ----
[02:15:08] thread 'rustc' panicked at 'value was not set', src/libcore/option.rs:1034:5
[02:15:08] thread 'rustc' panicked at 'value was not set', src/libcore/option.rs:1034:5
[02:15:08] thread 'string.rs - string::String (line 160)' panicked at 'Some expected error codes were not found: ["E0277"]', src/librustdoc/test.rs:313:9
[02:15:08] ---- sync.rs - sync::Arc (line 147) stdout ----
[02:15:08] thread 'rustc' panicked at 'value was not set', src/libcore/option.rs:1034:5
[02:15:08] thread 'sync.rs - sync::Arc (line 147)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:308:13
[02:15:08] 
---
[02:15:08] 
[02:15:08] error: test failed, to rerun pass '--doc'
[02:15:08] 
[02:15:08] 
[02:15:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[02:15:08] 
[02:15:08] 
[02:15:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[02:15:08] Build completed unsuccessfully in 0:24:32
[02:15:08] Build completed unsuccessfully in 0:24:32
[02:15:08] make: *** [check] Error 1
[02:15:08] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01bdcfa4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar 23 20:57:13 UTC 2019
---
travis_time:end:1a6d0838:start=1553374636264042940,finish=1553374636269913960,duration=5871020
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:066352a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19ff318b
travis_time:start:19ff318b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:147f155c
$ dmesg | grep -i kill
