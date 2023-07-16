plain
travis_time:end:03b26f80:start=1546678024535720618,finish=1546678025427659892,duration=891939274
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:12] .................................................................................................... 300/5293
[00:59:14] .................................................................................................... 400/5293
[00:59:18] .................................................................................................... 500/5293
[00:59:21] ...............................i.................................................................... 600/5293
[00:59:25] .................................................................................FF................. 700/5293
[00:59:36] .......................................................................i...............i............ 900/5293
[00:59:39] .................................................................................................iii 1000/5293
[00:59:43] ii.................................................................................................. 1100/5293
[00:59:45] .................................................................................................... 1200/5293
---
[01:01:52] .................................................................................................... 4800/5293
[01:01:57] .................................................................................................... 4900/5293
[01:02:00] .................................................................................................... 5000/5293
[01:02:06] .................................................................................................... 5200/5293
xperimental","highlight_start":29,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(no_core)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: no_core is experimental (see issue #29639)\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-invalid-2.rs:4:29\n   |\nLL | #![cfg_attr(broken, no_std, no_core)] //~ ERROR no_core is experimental\n   |                             ^^^^^^^\n   |\n   = help: add #![feature(no_core)] to the crate attributes to enable\n\n"}
[01:02:09] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:02:09] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:02:09] ------------------------------------------
[01:02:09] 
[01:02:09] thread '[ui] ui/conditional-compilation/cfg-attr-multi-invalid-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:02:09] 
---
[01:02:09] 
[01:02:09] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:02:09] 
[01:02:09] 
[01:02:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:09] 
[01:02:09] 
[01:02:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:09] Build completed unsuccessfully in 0:04:04
[01:02:09] Build completed unsuccessfully in 0:04:04
[01:02:09] Makefile:48: recipe for target 'check' failed
[01:kout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:030bae98
travis_time:start:030bae98
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03a44134
$ dmesg | grep -i kill
