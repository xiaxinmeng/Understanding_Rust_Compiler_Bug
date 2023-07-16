plain
travis_time:end:31204529:start=1551550249877092821,finish=1551550250820131648,duration=943038827
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
#################################################################         90.5%
######################################################################## 100.0%
[00:02:02] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:02]     Updating crates.io index
[00:02:14]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:16]   Downloaded lazy_static v0.2.11
[00:02:16]   Downloaded time v0.1.40
[00:02:16]   Downloaded num_cpus v1.8.0
[00:02:16]   Downloaded petgraph v0.4.13
---
tidy check
[00:04:09] * 569 error codes
[00:04:09] * highest error code: E0724
[00:04:10] * 252 features
[00:04:10] tidy error: The Unstable Book has a 'library feature' section 'test' which doesn't correspond to an unstable library feature
[00:04:11] invalid source: "git+https://github.com/gnzlbg/libtest#63ae76050ad3a2c81423f83d3bf4106ade630d4f"
[00:04:11] invalid source: "git+https://github.com/gnzlbg/libtest#63ae76050ad3a2c81423f83d3bf4106ade630d4f"
[00:04:11] some tidy checks failed
[00:04:11] 
[00:04:11] 
[00:04:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:11] 
[00:04:11] 
[00:04:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:11] Build completed unsuccessfully in 0:00:47
[00:04:11] Build completed unsuccessfully in 0:00:47
[00:04:11] make: *** [tidy] Error 1
[00:04:11] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1195c518
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  2 18:15:14 UTC 2019
---
travis_time:end:0655024e:start=1551550514968889083,finish=1551550514973845993,duration=4956910
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:050636e8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b03eb6a
travis_time:start:0b03eb6a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ca8cc90
$ dmesg | grep -i kill
