plain
[00:02:31] Successfully tagged rust-ci:latest
[00:02:31] Built container sha256:3815f133bcf0d8280e8d9e6d621ad74009c68c3a1829cce181c14c03e2bfc074
[00:02:31] Uploading finished image to s3://rust-lang-ci-sccache2/docker/6e066733b0b8bc03640758eb363f3a99027aa4f79b7978ff798327e892ff0aa1e2aeadb5cc8358d1817cc604e5021ce84d8e6dbf513140f592fbc78995b78ff4
[00:02:32] 
[00:02:32] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:37] xargs: docker: terminated by signal 13

[00:02:37] travis_time:end:31e59e92:start=1539914356570271497,finish=1539914456406667003,duration=99836395506
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:37] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---

[00:05:03] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:04] tidy error: /checkout/src/libsyntax/feature_gate.rs:1659: TODO is deprecated; use FIXME
[00:05:05] some tidy checks failed
[00:05:05] 
[00:05:05] 
[00:05:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:05] 
[00:05:05] 
[00:05:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:05] Build completed unsuccessfully in 0:00:48
[00:05:05] Build completed unsuccessfully in 0:00:48
[00:05:05] make: *** [tidy] Error 1
[00:05:05] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14b30aab
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1418e09e:start=1539914605265184490,finish=1539914605270092891,duration=4908401
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1abb73f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:27b25673
travis_time:start:27b25673
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3206525c
$ dmesg | grep -i kill
