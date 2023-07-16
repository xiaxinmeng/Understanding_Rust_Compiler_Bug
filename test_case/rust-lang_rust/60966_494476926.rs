plain
travis_time:end:1d52570c:start=1558453116087804491,finish=1558453202089290512,duration=86001486021
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:31] 
[01:17:31] running 143 tests
[01:17:33] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:17:35] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:17:35] 
[01:17:35]  finished in 4.492
[01:17:35] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:37] 
[01:17:37] running 9 tests
[01:17:37] iiiiiiiii
[01:17:37] 
[01:17:37]  finished in 0.149
[01:17:37] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:52] 
[01:17:52] running 122 tests
[01:18:17] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:18:22] .i.i......iii.i.....ii
[01:18:22] 
[01:18:22]  finished in 29.453
[01:18:22] travis_fold:end:test_debuginfo

---
[01:25:53]    Compiling rand_chacha v0.1.0
[01:25:53]    Compiling rand_pcg v0.1.1
[01:25:53]    Compiling rand v0.6.1
[01:25:57]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[01:26:03] error: duplicate diagnostic item found: `vec_type`.
[01:26:03]     |
[01:26:03] 294 | / pub struct Vec<T> {
[01:26:03] 295 | |     buf: RawVec<T>,
[01:26:03] 296 | |     len: usize,
---
[01:26:03] warning: build failed, waiting for other jobs to finish...
[01:27:14] error: build failed
[01:27:14] 
[01:27:14] 
[01:27:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:27:14] 
[01:27:14] 
[01:27:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:14] Build completed unsuccessfully in 0:21:00
[01:27:14] Build completed unsuccessfully in 0:21:00
[01:27:14] Makefile:48: recipe for target 'check' failed
[01:27:14] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f7f226c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 21 17:07:26 UTC 2019
---
travis_time:end:1ab1bf7e:start=1558458447795139596,finish=1558458447800308954,duration=5169358
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05bb7f7c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e0acf40
travis_time:start:0e0acf40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03d35322
$ dmesg | grep -i kill
