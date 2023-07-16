plain
travis_time:end:1084d672:start=1551455969973713154,finish=1551455970830388359,duration=856675205
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
[01:13:21] 
[01:13:21] running 119 tests
[01:13:45] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:13:50] i......iii.i.....ii
[01:13:50] 
[01:13:50]  finished in 28.804
[01:13:50] travis_fold:end:test_debuginfo

---
[01:35:46] 
[01:35:46] error: test failed, to rerun pass '--lib'
[01:35:46] 
[01:35:46] 
[01:35:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:35:46] 
[01:35:46] 
[01:35:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:46] Build completed unsuccessfully in 0:33:57
[01:35:46] Build completed unsuccessfully in 0:33:57
[01:35:46] make: *** [check] Error 1
[01:35:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16f60203
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar  1 17:35:29 UTC 2019
---
travis_time:end:1d3bf98c:start=1551461730712359756,finish=1551461730716932921,duration=4573165
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2b4f048e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:007a1af0
travis_time:start:007a1af0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:21e4cfa2
$ dmesg | grep -i kill
