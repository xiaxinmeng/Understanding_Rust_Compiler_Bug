plain
travis_time:end:0c2323e0:start=1556020801833582478,finish=1556020901479868523,duration=99646286045
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:42] 
[01:12:42] running 9 tests
[01:12:42] iiiiiiiii
[01:12:42] 
[01:12:42]  finished in 0.150
[01:12:42] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:58] 
[01:12:58] running 121 tests
[01:13:23] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:13:27] i.i......iii.i.....ii
[01:13:27] 
[01:13:27]  finished in 29.368
[01:13:27] travis_fold:end:test_debuginfo

---
[01:41:20] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0594 (line 9891) stdout ----
[01:41:20] error: expected type, found `}`
[01:41:20]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:9895:1
[01:41:20]   |
[01:41:20] 5 | https://doc.rust-lang.org/stable/rust-by-example/custom_types/constants.html
[01:41:20]   |      - help: try using a semicolon: `;`
[01:41:20]   | ^ expecting a type here because of type ascription
[01:41:20] 
[01:41:20] error: aborting due to previous error
[01:41:20] 
---
[01:41:20] 
[01:41:20] 
[01:41:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:20] Build completed unsuccessfully in 0:39:58
[01:41:20] Makefile:48: recipe for target 'check' failed
[01:41:20] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d5ac4f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 13:43:11 UTC 2019
---
travis_time:end:1381c091:start=1556026992659289065,finish=1556026992664021705,duration=4732640
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:191e0f64
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06214ba8
travis_time:start:06214ba8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2d332ffa
$ dmesg | grep -i kill
