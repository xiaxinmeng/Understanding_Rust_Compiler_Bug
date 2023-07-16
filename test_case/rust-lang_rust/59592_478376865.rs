plain
travis_time:end:0cb509e0:start=1554062098345538028,finish=1554062099345541414,duration=1000003386
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
#####################################################################     96.5%
######################################################################## 100.0%
[00:01:57] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:57]     Updating crates.io index
[00:02:12]     Updating git repository `https://github.com/tkaitchuck/aHash.git`
[00:02:13]   Downloaded num_cpus v1.8.0
[00:02:13]   Downloaded filetime v0.2.4
[00:02:13]   Downloaded petgraph v0.4.13
[00:02:13]   Downloaded cmake v0.1.33
---
[00:04:15] * fuchsia-cprng 
[00:04:15] * proc-macro-hack 
[00:04:15] * rand_jitter 
[00:04:15] * rand_os 
[00:04:15] * rdrand 
[00:04:15] invalid source: "git+https://github.com/tkaitchuck/aHash.git#91bcb84a5cee8dfc7782cc815c3c7f9c1b88badc"
[00:04:15] some tidy checks failed
[00:04:15] 
[00:04:15] 
[00:04:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:15] 
[00:04:15] 
[00:04:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:15] Build completed unsuccessfully in 0:00:50
[00:04:15] Build completed unsuccessfully in 0:00:50
[00:04:15] Makefile:67: recipe for target 'tidy' failed
[00:04:15] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d547dcc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 31 19:59:27 UTC 2019
---
travis_time:end:0127fae2:start=1554062368161187896,finish=1554062368167051623,duration=5863727
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b38e455
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:053c8bcf
travis_time:start:053c8bcf
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03598592
$ dmesg | grep -i kill
