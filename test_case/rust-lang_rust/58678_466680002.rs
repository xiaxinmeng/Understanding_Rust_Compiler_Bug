plain
travis_time:end:0073247a:start=1550947718577478137,finish=1550947794302070082,duration=75724591945
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:59] tidy error: /checkout/src/libsyntax/parse/parser.rs:8310: trailing whitespace
[00:04:00] some tidy checks failed
[00:04:00] 
[00:04:00] 
[00:04:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:00] 
[00:04:00] 
[00:04:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:00] Build completed unsuccessfully in 0:00:46
[00:04:00] Build completed unsuccessfully in 0:00:46
[00:04:00] make: *** [tidy] Error 1
[00:04:00] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c9f3474
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb 23 18:54:04 UTC 2019
---
travis_time:end:02297160:start=1550948045154431333,finish=1550948045159060687,duration=4629354
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014fed76
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08f90271
travis_time:start:08f90271
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00439b80
$ dmesg | grep -i kill
