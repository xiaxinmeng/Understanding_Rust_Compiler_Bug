plain
[00:02:22] Successfully tagged rust-ci:latest
[00:02:22] Built container sha256:dd90dd407f97ba4becdf78a08beb58049466d5beb7940490e2be217d20a47eb5
[00:02:22] Uploading finished image to s3://rust-lang-ci-sccache2/docker/254673b822b84233b3d8510170c1c331460198676199202c3bf8d1c7c9e0e1b16875995a19ea0d0d82a3789a612f21c840d7e6245ccdc54916e891d31de24244
[00:02:23] 
[00:02:23] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:30] xargs: docker: terminated by signal 13

[00:02:30] travis_time:end:23e753d6:start=1532651782794697198,finish=1532651921152987922,duration=138358290724
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:30] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---

[00:05:05] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:06] tidy error: /checkout/src/test/ui/rfc-2093-infer-outlives/infer-static.rs: too many trailing newlines (3)
[00:05:06] tidy error: Found 1 features without a gate test.
[00:05:06] Expected a gate test for the feature 'infer_static_outlives_requirements'.
[00:05:06] Hint: create a failing test file named 'feature-gate-infer_static_outlives_requirements.rs'
[00:05:06]       in the 'ui' test suite, with its failures due to
[00:05:06]       missing usage of #![feature(infer_static_outlives_requirements)].
[00:05:06] Hint: If you already have such a test and don't want to rename it,
[00:05:06]       you can also add a // gate-test-infer_static_outlives_requirements line to the test file.
[00:05:07] some tidy checks failed
[00:05:07] 
[00:05:07] 
[00:05:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:07] 
[00:05:07] 
[00:05:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:07] Build completed unsuccessfully in 0:00:46
[00:05:07] Build completed unsuccessfully in 0:00:46
[00:05:07] make: *** [tidy] Error 1
[00:05:07] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:097c1b68
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:003ff90c:start=1532652078826109110,finish=1532652078833963843,duration=7854733
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02f07783
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1993fa58
travis_time:start:1993fa58
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10c22f60
$ dmesg | grep -i kill
