plain
travis_time:end:22a350e8:start=1559887026386601973,finish=1559887027181363321,duration=794761348
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:03:31]   Downloaded dlmalloc v0.1.3
[00:03:32]   Downloaded rustc-ap-rustc_cratesio_shim v407.0.0
[00:03:32]   Downloaded bytecount v0.5.1
[00:03:32]   Downloaded unicode_categories v0.1.1
[00:03:32]   Downloaded rgb v0.8.13
[00:03:32]   Downloaded atty v0.2.11
[00:03:32]   Downloaded net2 v0.2.33
[00:03:32]   Downloaded git2 v0.8.0
[00:03:32]   Downloaded rustc-rayon v0.1.2
---
[00:04:56] * rdrand 
[00:04:56] some tidy checks failed
[00:04:56] 
[00:04:56] 
[00:04:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:56] 
[00:04:56] 
[00:04:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:56] Build completed unsuccessfully in 0:01:14
---
travis_time:end:053fb0a1:start=1559887335012305113,finish=1559887335017242139,duration=4937026
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23a3e3ac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b0de14a
travis_time:start:2b0de14a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:012289e0
$ dmesg | grep -i kill
