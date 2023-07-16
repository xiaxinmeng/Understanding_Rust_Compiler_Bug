plain
travis_time:end:06c91bf0:start=1551932172688926405,finish=1551932173597035017,duration=908108612
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:00]  ---> 5a459b5b6133
[00:02:00] Successfully built 5a459b5b6133
[00:02:00] Successfully tagged rust-ci:latest
[00:02:00] Built container sha256:5a459b5b6133c303505bda700757809a29e55343b4a3b83d1f7cae08c62dcfd8
[00:02:00] Uploading finished image to s3://rust-lang-ci-sccache2/docker/3010b440ec30a8a3a8347df228d54e1ca9d78fcce0ba5b618ad3c3d0ade5b8a5f2d5b466d856e4e2587679d1e7f6f47e49cd9fa80519b41565b1bbc2f1be23e2
[00:02:50] upload failed: - to s3://rust-lang-ci-sccache2/docker/3010b440ec30a8a3a8347df228d54e1ca9d78fcce0ba5b618ad3c3d0ade5b8a5f2d5b466d856e4e2587679d1e7f6f47e49cd9fa80519b41565b1bbc2f1be23e2 Unable to locate credentials

[00:02:50] travis_time:end:00c4f43c:start=1551932197322908505,finish=1551932354505169632,duration=157182261127
[CI_JOB_NAME=x86_64-gnu-llvm-6.0]
[00:02:50] [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
---

[00:05:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:09] tidy error: /checkout/src/ci/docker/dist-powerpc64-linux/build-powerpc64-toolchain.sh:26: line longer than 100 chars
[00:05:11] some tidy checks failed
[00:05:11] 
[00:05:11] 
[00:05:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:11] 
[00:05:11] 
[00:05:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:11] Build completed unsuccessfully in 0:00:45
[00:05:11] Build completed unsuccessfully in 0:00:45
[00:05:11] make: *** [tidy] Error 1
[00:05:11] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09a0ed6e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar  7 04:21:36 UTC 2019
---
travis_time:end:37b033e4:start=1551932496795864106,finish=1551932496800640516,duration=4776410
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:003ffaa8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e354676
travis_time:start:1e354676
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:008d9168
$ dmesg | grep -i kill
