plain
travis_time:end:03079aba:start=1560023244962839850,finish=1560023342162488535,duration=97199648685
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:01:33] .....................ii............................................................................. 1400/2925
[01:01:45] .................................................................................................... 1500/2925
[01:01:54] .........................................................................i.......i.................. 1600/2925
[01:02:08] .................................................................................................... 1700/2925
[01:02:22] ..............................F..................................................................... 1800/2925
[01:02:46] ....i.......................................................................i....................... 2000/2925
[01:03:12] .................................................................................................... 2100/2925
[01:03:34] ........................................................................................test [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
[01:03:35] ............ 2200/2925
---
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:05:24]   left: `"string\r\nliteral"`,
[01:05:24]  right: `"string\nliteral"`', /checkout/src/test/run-pass/lexer-crlf-line-endings-string-literal-doc-comment.rs:32:5
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] 
---
[01:05:24] test result: FAILED. 2915 passed; 1 failed; 9 ignored; 0 measured; 0 filtered out
[01:05:24] 
[01:05:24] 
[01:05:24] 
[01:05:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:24] 
[01:05:24] 
[01:05:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:24] Build completed unsuccessfully in 1:01:29
---
travis_time:end:0b1d0db8:start=1560027277780394948,finish=1560027277840754842,duration=60359894
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20f16e6d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1104b394
$ dmesg | grep -i kill
