plain
travis_time:end:00059ed4:start=1553877351100236139,finish=1553877352281280019,duration=1181043880
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
[01:26:25] 
[01:26:25] running 9 tests
[01:26:25] iiiiiiiii
[01:26:25] 
[01:26:25]  finished in 0.173
[01:26:25] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:26:42] 
[01:26:42] running 120 tests
[01:27:12] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:27:18] .i......iii.i.....ii
[01:27:18] 
[01:27:18]  finished in 36.026
[01:27:18] travis_fold:end:test_debuginfo

---
[01:56:23] travis_fold:end:stage0-linkchecker

[01:56:23] travis_time:end:stage0-linkchecker:start=1553884343946150840,finish=1553884346142709293,duration=2196558453

[01:56:25] std/io/trait.Write.html:44: broken link fragment `#tymethod.write_all` pointing to `std/io/trait.Write.html`
[01:56:35] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:56:35] 
[01:56:35] 
[01:56:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:56:35] expected success, got: exit code: 101
[01:56:35] expected success, got: exit code: 101
[01:56:35] 
[01:56:35] 
[01:56:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:56:35] Build completed unsuccessfully in 0:43:22
[01:56:35] make: *** [check] Error 1
[01:56:35] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01997a6c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 29 18:32:38 UTC 2019
---
travis_time:end:0be56177:start=1553884361204163116,finish=1553884361210860453,duration=6697337
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:012f200a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08b0df52
travis_time:start:08b0df52
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08f1485a
$ dmesg | grep -i kill
