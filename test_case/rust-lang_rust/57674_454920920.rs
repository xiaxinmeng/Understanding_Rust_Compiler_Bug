plain
travis_time:end:008ab739:start=1547664771180217585,finish=1547664772216389000,duration=1036171415
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:11:39] ...................................i................................................................ 2400/2942
[01:11:51] .................................................................................................... 2500/2942
[01:12:23] .................................................................................................... 2600/2942
[01:12:33] .................................................................................................... 2700/2942
[01:12:43] ....................................F............................................................... 2800/2942
[01:13:00] ..........................................
[01:13:00] failures:
[01:13:00] 
[01:13:00] ---- [run-pass] run-pass/type-id-higher-rank.rs stdout ----
---
[01:13:00] ------------------------------------------
[01:13:00] stderr:
[01:13:00] ------------------------------------------
[01:13:00] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:13:00]   left: `TypeId { t: 15076019611076229820 }`,
[01:13:00]  right: `TypeId { t: 14349794804050306325 }`', /checkout/src/test/run-pass/type-id-higher-rank.rs:21:9
[01:13:00] 
[01:13:00] ------------------------------------------
[01:13:00] 
[01:13:00] thread '[run-pass] run-pass/type-id-higher-rank.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:13:00] 
[01:13:00] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:13:00] 
[01:13:00] 
[01:13:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:00] 
[01:13:00] 
[01:13:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:00] Build completed unsuccessfully in 0:10:29
[01:13:00] Build completed unsuccessfully in 0:10:29
[01:13:00] Makefile:48: recipe for target 'check' failed
[01:13:00] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05d64965
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 16 20:06:03 UTC 2019
---
172064 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib
162608 ./obj/build/bootstrap/debug/incremental
153292 ./src/tools/clang
146492 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405
146488 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405/s-f8lnhxnjs3-m28ewj-3sgufgpmw18gi
139036 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
139032 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
136692 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
124932 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
---
56896 ./src/llvm/test/MC
56100 ./obj/build/x86_64-unknown-linux-gnu/test/run-pass/proc-macro
56088 ./.git/modules/src/tools
54428 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build
"$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:054d476f
travis_time:start:054d476f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01f9b92d
$ dmesg | grep -i kill
