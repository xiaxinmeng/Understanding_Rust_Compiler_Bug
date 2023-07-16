plain
travis_time:end:2d6023b3:start=1552210773351447243,finish=1552210851930414874,duration=78578967631
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:38] Successfully built 7abb4d8a4fbc
[00:01:38] Successfully tagged rust-ci:latest
[00:01:38] Built container sha256:7abb4d8a4fbcea5d7f850abcb0035f319929b0457004add8d521a8536f42d7b7
[00:01:38] Uploading finished image to s3://rust-lang-ci-sccache2/docker/974354dbf162603374cfe59000ce5a1f3487f8bda7275fa6ae6db03162d61772b61dec7c27a7d1b5b0c6d507478814e49210f55ac865fa7005090c32ce135c50
[00:02:26] upload failed: - to s3://rust-lang-ci-sccache2/docker/974354dbf162603374cfe59000ce5a1f3487f8bda7275fa6ae6db03162d61772b61dec7c27a7d1b5b0c6d507478814e49210f55ac865fa7005090c32ce135c50 Unable to locate credentials

[00:02:26] travis_time:end:180c037e:start=1552210874347216490,finish=1552211007745489939,duration=133398273449
[CI_JOB_NAME=x86_64-gnu-llvm-6.0]
[00:02:26] [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
---

[00:05:08] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:08] tidy error: /checkout/src/tools/publish_toolstate.py:25: line longer than 100 chars
[00:05:10] some tidy checks failed
[00:05:10] 
[00:05:10] 
[00:05:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:10] 
[00:05:10] 
[00:05:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:10] Build completed unsuccessfully in 0:00:57
[00:05:10] Build completed unsuccessfully in 0:00:57
[00:05:10] make: *** [tidy] Error 1
[00:05:10] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:23a8ecb8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 10 09:46:12 UTC 2019
---
travis_time:end:094ba722:start=1552211173350730371,finish=1552211173358550689,duration=7820318
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10a03803
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dad09c4
travis_time:start:0dad09c4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e366f6e
$ dmesg | grep -i kill
