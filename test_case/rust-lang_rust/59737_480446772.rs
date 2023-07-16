plain
[00:21:46] [RUSTC-TIMING] rustc_errors test:false 17.027
[00:22:53] [RUSTC-TIMING] syntax test:false 66.832
[00:22:53]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:23:30] [RUSTC-TIMING] syntax_ext test:false 37.196
[00:24:25] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:26:35] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:29:20] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:29:31]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:29:31]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:31:17] [RUSTC-TIMING] rustc_incremental test:false 105.779
[00:31:17] [RUSTC-TIMING] rustc_incremental test:false 105.779
[00:32:45] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:35:37] [RUSTC-TIMING] rustc_typeck test:false 365.590
[00:35:37]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:36:51] [RUSTC-TIMING] rustc_mir test:false 439.377
[00:36:53]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
---
[00:38:19]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:38:20] [RUSTC-TIMING] rustc_traits test:false 87.521
[00:38:20]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:39:21] [RUSTC-TIMING] rustc_borrowck test:false 62.471
[00:39:25] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:39:33] [RUSTC-TIMING] rustc_privacy test:false 74.442
[00:39:49] [RUSTC-TIMING] rustc_codegen_utils test:false 91.060
[00:39:49]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:39:49]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:39:49]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:40:40] [RUSTC-TIMING] rustc_save_analysis test:false 50.739
[00:41:10] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 139.
travis_time:start:152521f4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr  5 22:55:16 UTC 2019
---
travis_time:end:1e73684a:start=1554504918314433028,finish=1554504918328971465,duration=14538437
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:002afdd6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01b26818
travis_time:start:01b26818
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11a3842b
$ dmesg | grep -i kill
