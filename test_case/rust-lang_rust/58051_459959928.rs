plain
travis_time:end:08a94ecf:start=1549103236743417801,finish=1549103308760072175,duration=72016654374
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
[01:10:02] 
[01:10:02] running 119 tests
[01:10:28] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:10:32] i......iii.i.....ii
[01:10:32] 
[01:10:32]  finished in 29.606
[01:10:32] travis_fold:end:test_debuginfo

---
[01:36:08] travis_fold:end:stage0-linkchecker

[01:36:08] travis_time:end:stage0-linkchecker:start=1549109083829891554,finish=1549109085905854123,duration=2075962569

[01:36:15] core/str/struct.EscapeDefault.html:1: broken link - core/str/primitive.str.html
[01:36:15] core/str/index.html:10: broken link - core/str/primitive.str.html
[01:36:15] core/str/index.html:11: broken link - core/str/primitive.str.html
[01:36:15] core/str/index.html:12: broken link - core/str/primitive.str.html
[01:36:15] core/str/struct.EscapeUnicode.html:1: broken link - core/str/primitive.str.html
[01:36:15] core/str/struct.EscapeDebug.html:1: broken link - core/str/primitive.str.html
[01:36:15] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:39:9
[01:36:15] 
[01:36:15] 
[01:36:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:36:15] expected success, got: exit code: 101
[01:36:15] expected success, got: exit code: 101
[01:36:15] 
[01:36:15] 
[01:36:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:15] Build completed unsuccessfully in 0:37:59
[01:36:15] Makefile:48: recipe for target 'check' failed
[01:36:15] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10b88872
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  2 12:04:54 UTC 2019
---
travis_time:end:08d76c70:start=1549109095660083263,finish=1549109095666728655,duration=6645392
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10326bc0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06d5f728
travis_time:start:06d5f728
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d5829d0
$ dmesg | grep -i kill
