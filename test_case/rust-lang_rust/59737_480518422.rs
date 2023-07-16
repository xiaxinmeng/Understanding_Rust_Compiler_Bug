plain
[00:22:18] [RUSTC-TIMING] rustc_errors test:false 16.545
[00:23:28] [RUSTC-TIMING] syntax test:false 69.964
[00:23:28]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:24:08] [RUSTC-TIMING] syntax_ext test:false 39.402
[00:29:15] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:30:31]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:30:31]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:32:20] [RUSTC-TIMING] rustc_incremental test:false 109.688
[00:32:20] [RUSTC-TIMING] rustc_incremental test:false 109.688
[00:33:10] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:35:19] [RUSTC-TIMING] rustc_metadata test:false 287.970
[00:35:19]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:35:51] [RUSTC-TIMING] rustc_allocator test:false 33.752
[00:35:51] [RUSTC-TIMING] rustc_allocator test:false 33.752
[00:36:15] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:37:26] [RUSTC-TIMING] rustc_typeck test:false 415.312
[00:37:26] [RUSTC-TIMING] rustc_typeck test:false 415.312
[00:38:00] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 139.
travis_time:start:007926b6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr  6 16:38:50 UTC 2019
---
travis_time:end:054b09b0:start=1554568732179907425,finish=1554568732195445800,duration=15538375
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c01e351
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fa9e45f
travis_time:start:0fa9e45f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:33d8e9a0
$ dmesg | grep -i kill
