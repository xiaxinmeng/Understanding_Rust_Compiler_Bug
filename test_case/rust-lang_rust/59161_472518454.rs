plain
travis_time:end:110ab694:start=1552492657611430488,finish=1552492744743887691,duration=87132457203
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
Setting environment variables from .travis.yml
$ export CI_JOB_NAME=$TRAVIS_JOB_NAME
$ export RUST_BACKTRACE=1
$ bash -c 'echo $BASH_VERSION'
4.3.48(1)-release
travis_fold:start:before_install.1
---
[00:02:00]  ---> 7c0efeafa58e
[00:02:00] Successfully built 7c0efeafa58e
[00:02:00] Successfully tagged rust-ci:latest
[00:02:00] Built container sha256:7c0efeafa58e1b9db6c334b474c73b0869cee206cfcb66de33b8d99697b73836
[00:02:00] Uploading finished image to s3://rust-lang-ci-sccache2/docker/8dcfaffd04024de90126ffc81fcdb56fbaa8e5ec2fe8eae50ae68cfe40ef281bfe4085b2d7cde31c578de2a90470847e278efbef6e401173da016643f489dbaa
[00:02:48] upload failed: - to s3://rust-lang-ci-sccache2/docker/8dcfaffd04024de90126ffc81fcdb56fbaa8e5ec2fe8eae50ae68cfe40ef281bfe4085b2d7cde31c578de2a90470847e278efbef6e401173da016643f489dbaa Unable to locate credentials

[00:02:49] travis_time:end:1b48d4ea:start=1552492778857228772,finish=1552492924840488221,duration=145983259449
[CI_JOB_NAME=x86_64-gnu-llvm-6.0]
[00:02:49] [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
---
[01:12:20] .................................................................................................... 4400/5473
[01:12:23] .................................................................................................... 4500/5473
[01:12:26] .................................................................................................... 4600/5473
[01:12:31] ...............i.................................................................................... 4700/5473
[01:12:36] ...............................F.................................................................... 4800/5473
[01:12:43] .................................................................................................... 5000/5473
[01:12:47] .................................................................................................... 5100/5473
[01:12:51] .................................................................................................... 5200/5473
[01:12:54] .................................................................................................... 5300/5473
---
[01:12:59] 
[01:12:59] 1 error: cannot use a built-in attribute through an import
[01:12:59] 2   --> $DIR/cross-crate.rs:7:3
[01:12:59] 3    |
[01:12:59] - LL | #[built_in_attr] //~ ERROR cannot use a built-in attribute through an import
[01:12:59] + LL | #[built_in_attr]
[01:12:59] 6    |
[01:12:59] 7 note: the built-in attribute imported here
[01:12:59] 
[01:12:59] 13 error: cannot use a tool module through an import
[01:12:59] 13 error: cannot use a tool module through an import
[01:12:59] 14   --> $DIR/cross-crate.rs:8:3
[01:12:59] 15    |
[01:12:59] - LL | #[tool_mod::skip] //~ ERROR cannot use a tool module through an import
[0gh an import","highlight_start":3,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the tool module imported here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs","byte_start":75,"byte_end":89,"line_start":5,"line_end":5,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"use cross_crate::*;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot use a tool module through an import\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs:8:3\n   |\nLL | #[tool_mod::skip] //~ ERROR cannot use a tool module through an import\n   |   ^^^^^^^^\n   |\nnote: the tool module imported here\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs:5:5\n   |\nLL | use cross_crate::*;\n   |     ^^^^^^^^^^^^^^\n\n"}
[01:12:59] 
[01:12:59] ------------------------------------------
[01:12:59] 
[01:12:59] thread '[ui] ui/rust-2018/uniform-paths/cross-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:12:59] 
[01:12:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:12:59] 
[01:12:59] 
[01:12:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:59] 
[01:12:59] 
[01:12:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:59] Build completed unsuccessfully in 0:04:19
---
travis_time:end:01827c80:start=1552497137285392104,finish=1552497137290354785,duration=4962681
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:272e1cb0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:011824dc
travis_time:start:011824dc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:010a0e24
$ dmesg | grep -i kill
