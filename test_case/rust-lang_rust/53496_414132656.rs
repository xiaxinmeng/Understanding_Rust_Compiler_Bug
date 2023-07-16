plain
[00:01:59] Successfully tagged rust-ci:latest
[00:01:59] Built container sha256:6f7d872ce5e67b0bcd6ed114d4bf31d383daadb7417388c0d70966805b9cd8ff
[00:01:59] Uploading finished image to s3://rust-lang-ci-sccache2/docker/3fac661d52d4c71ac16bc8a580645b8477f900bb38ec96582412bcffa9d55c97d34b20dca6548234f73ee650dcc2ed336a3cebba51a609d3ed1a29ba737d52be
[00:01:59] 
[00:01:59] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:06] xargs: docker: terminated by signal 13

[00:02:06] travis_time:end:03b4ebe0:start=1534689352554980893,finish=1534689422704258072,duration=70149277179
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:07] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---

[00:04:57] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:57] tidy error: /checkout/src/librustc_mir/borrow_check/nll/invalidation.rs:544: TODO is deprecated; use FIXME
[00:04:57] tidy error: /checkout/src/libsyntax/parse/parser.rs:3538: line longer than 100 chars
[00:04:58] some tidy checks failed
[00:04:58] 
[00:04:58] 
[00:04:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:58] 
[00:04:58] 
[00:04:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:58] Build completed unsuccessfully in 0:00:52
[00:04:58] Build completed unsuccessfully in 0:00:52
[00:04:58] Makefile:79: recipe for target 'tidy' failed
[00:04:58] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05d6daef
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:12e71f50:start=1534689595455853458,finish=1534689595462513810,duration=6660352
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f54256c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bff2cd8
travis_time:start:1bff2cd8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0733c494
$ dmesg | grep -i kill
