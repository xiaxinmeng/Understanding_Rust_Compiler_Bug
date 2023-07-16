plain
travis_time:end:0ed9807d:start=1561279419286836850,finish=1561279420195328034,duration=908491184
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
####################################################################      95.8%
######################################################################## 100.0%
[00:02:03] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:03]     Updating crates.io index
[00:02:22]     Updating git repository `https://github.com/Zoxc/mimallocator.git`
[00:02:24]   Downloaded cc v1.0.35
[00:02:24]   Downloaded filetime v0.2.4
[00:02:24]   Downloaded num_cpus v1.8.0
[00:02:24]   Downloaded serde v1.0.92
---
tidy check
[00:04:46] * 576 error codes
[00:04:46] * highest error code: E0731
[00:04:47] * 260 features
[00:04:50] invalid source: "git+https://github.com/Zoxc/mimallocator.git#7be315f3b8edb3a0536b63e608a966ff33898a7b"
[00:04:50] invalid source: "git+https://github.com/Zoxc/mimallocator.git#7be315f3b8edb3a0536b63e608a966ff33898a7b"
[00:04:50] some tidy checks failed
[00:04:50] 
[00:04:50] 
[00:04:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:50] 
[00:04:50] 
[00:04:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:50] Build completed unsuccessfully in 0:01:13
---
travis_time:end:045065b8:start=1561279721770369764,finish=1561279721774938627,duration=4568863
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:159fd0b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:064c42c9
travis_time:start:064c42c9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0532fd01
$ dmesg | grep -i kill
