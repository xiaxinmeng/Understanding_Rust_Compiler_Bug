plain
travis_time:end:0d34612f:start=1551466573845139466,finish=1551466576196769896,duration=2351630430
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:54] 
######################################################################## 100.0%
[00:01:54] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:54]     Updating crates.io index
[00:02:07]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:08]   Downloaded num_cpus v1.8.0
[00:02:08]   Downloaded serde_json v1.0.33
[00:02:08]   Downloaded time v0.1.40
[00:02:08]   Downloaded filetime v0.2.4
---
tidy check
[00:04:04] * 569 error codes
[00:04:04] * highest error code: E0724
[00:04:05] * 252 features
[00:04:05] invalid source: "git+https://github.com/mati865/compiletest-rs.git?branch=rustup#2bf88c77e56a873a7d90dc5e2c8efeb0e2dc9d00"
[00:04:05] some tidy checks failed
[00:04:05] 
[00:04:05] 
[00:04:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:05] 
[00:04:05] 
[00:04:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:05] Build completed unsuccessfully in 0:00:48
[00:04:05] Build completed unsuccessfully in 0:00:48
[00:04:05] make: *** [tidy] Error 1
[00:04:05] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:091c49d2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar  1 19:00:32 UTC 2019
---
travis_time:end:014da2b4:start=1551466833753090548,finish=1551466833758668097,duration=5577549
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08de0a2a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e1336da
travis_time:start:1e1336da
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fac0a85
$ dmesg | grep -i kill
