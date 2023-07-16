plain
[00:02:21] Successfully tagged rust-ci:latest
[00:02:21] Built container sha256:1d7c006d094f57500ef741cfcdefce332a6c2535299a37e1745e3fe9f3779059
[00:02:21] Uploading finished image to s3://rust-lang-ci-sccache2/docker/254673b822b84233b3d8510170c1c331460198676199202c3bf8d1c7c9e0e1b16875995a19ea0d0d82a3789a612f21c840d7e6245ccdc54916e891d31de24244
[00:02:21] 
[00:02:21] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:28] xargs: docker: terminated by signal 13

[00:02:29] travis_time:end:2e7d28a3:start=1532646242223917807,finish=1532646379639474800,duration=137415556993
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:29] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---

[00:05:05] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:05] tidy error: /checkout/src/librustc/session/config.rs:330: line longer than 100 chars
[00:05:06] some tidy checks failed
[00:05:06] 
[00:05:06] 
[00:05:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:06] 
[00:05:06] 
[00:05:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:06] Build completed unsuccessfully in 0:00:45
[00:05:06] Build completed unsuccessfully in 0:00:45
[00:05:06] Makefile:79: recipe for target 'tidy' failed
[00:05:06] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10ebda1c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:190314fa:start=1532646538108262966,finish=1532646538114369686,duration=6106720
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1520da96
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0171a808
travis_time:start:0171a808
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3194f11e
$ dmesg | grep -i kill
