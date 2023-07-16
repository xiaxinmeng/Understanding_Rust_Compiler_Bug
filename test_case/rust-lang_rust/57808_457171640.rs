plain
travis_time:end:2c3b2348:start=1548325289228463910,finish=1548325360552509876,duration=71324045966
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
[01:09:10] 
[01:09:10] running 118 tests
[01:09:34] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:09:38] ......iii.i.....ii
[01:09:38] 
[01:09:38]  finished in 27.846
[01:09:38] travis_fold:end:test_debuginfo

---
[01:34:28] travis_fold:end:stage0-linkchecker

[01:34:28] travis_time:end:stage0-linkchecker:start=1548331035103583688,finish=1548331037043818817,duration=1940235129

[01:34:30] std/arch/index.html:151: broken link - std/arch/x86/index.html
[01:34:30] std/arch/index.html:153: broken link - std/arch/arm/index.html
[01:34:30] std/arch/index.html:154: broken link - std/arch/aarch64/index.html
[01:34:30] std/arch/index.html:155: broken link - std/arch/mips/index.html
[01:34:30] std/arch/index.html:156: broken link - std/arch/mips64/index.html
[01:34:30] std/arch/index.html:157: broken link - std/arch/powerpc/index.html
[01:34:30] std/arch/index.html:158: broken link - std/arch/powerpc64/index.html
[01:34:30] std/arch/index.html:159: broken link - std/arch/nvptx/index.html
[01:34:30] std/arch/index.html:160: broken link - std/arch/wasm32/index.html
[01:34:36] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:39:9
[01:34:36] 
[01:34:36] 
[01:34:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:34:36] expected success, got: exit code: 101
[01:34:36] expected success, got: exit code: 101
[01:34:36] 
[01:34:36] 
[01:34:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:36] Build completed unsuccessfully in 0:36:40
[01:34:36] Makefile:48: recipe for target 'check' failed
[01:34:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:074f73e5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 24 11:57:25 UTC 2019
---
travis_time:end:3c740f50:start=1548331046650156924,finish=1548331046654829814,duration=4672890
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c90497e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10324e0a
travis_time:start:10324e0a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a7cdb25
$ dmesg | grep -i kill
