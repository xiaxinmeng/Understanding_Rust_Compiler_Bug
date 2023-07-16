plain
travis_time:end:10bc2efe:start=1548187995177865080,finish=1548188081514306979,duration=86336441899
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:22] 
######################################################################## 100.0%
[00:01:22] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:22]     Updating crates.io index
[00:01:32]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:04:04]   Downloaded cmake v0.1.33
[00:04:04]   Downloaded serde_derive v1.0.81
[00:04:04]   Downloaded time v0.1.40
[00:04:04]   Downloaded toml v0.4.10
---

[00:05:59] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:59] tidy error: /checkout/src/libtest/lib.rs:1: TODO is deprecated; use FIXME
[00:06:01] invalid source: "git+https://github.com/gnzlbg/libtest#01f857bc49ce423bae84368226a17854c47c109a"
[00:06:01] invalid source: "git+https://github.com/gnzlbg/libtest#01f857bc49ce423bae84368226a17854c47c109a"
[00:06:01] some tidy checks failed
[00:06:01] 
[00:06:01] 
[00:06:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:01] 
[00:06:01] 
[00:06:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:01] Build completed unsuccessfully in 0:00:48
[00:06:01] Build completed unsuccessfully in 0:00:48
[00:06:01] Makefile:69: recipe for target 'tidy' failed
[00:06:01] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0baf7eb6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 22 20:20:51 UTC 2019
---
travis_time:end:1695083d:start=1548188452043108450,finish=1548188452047722788,duration=4614338
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:38ecf0cb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ea59972
travis_time:start:0ea59972
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bc74ed9
$ dmesg | grep -i kill
