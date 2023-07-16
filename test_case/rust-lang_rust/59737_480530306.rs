plain
[00:17:25] travis_fold:end:build_docker

[00:17:26] + export SHELL=/bin/bash
[00:17:26] + exec /checkout/src/ci/run.sh
[00:17:26] + nohup nohup emulator @armeabi-v7a-18 -engine classic -no-window -no-audio -partition-size 2047
[00:17:26] travis_time:end:01fa5bf3:start=1554576355533195712,finish=1554577299818375754,duration=944285180042
[CI_JOB_NAME=arm-android]
[00:17:26] [CI_JOB_NAME=arm-android]
[00:17:26] Starting sccache server...
---
[00:21:35] [RUSTC-TIMING] syntax_pos test:false 5.757
[00:21:39]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:21:48] [RUSTC-TIMING] rustc_target test:false 18.687
[00:21:52] [RUSTC-TIMING] rustc_errors test:false 13.177
[00:22:46] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105002 ms
[00:22:58]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:23:35] [RUSTC-TIMING] syntax_ext test:false 37.261
[00:23:35] [RUSTC-TIMING] syntax_ext test:false 37.261
[00:25:21] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:29:01] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:29:38]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:29:38]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:29:38]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:30:46] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 139.
travis_time:start:2d427a31
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr  6 19:15:01 UTC 2019
---
travis_time:end:0681158d:start=1554578102955893971,finish=1554578102975312106,duration=19418135
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:098cfd72
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dad8aba
travis_time:start:0dad8aba
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:005eaf40
$ dmesg | grep -i kill
