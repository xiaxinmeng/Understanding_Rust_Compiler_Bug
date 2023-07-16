plain
[00:14:15] [RUSTC-TIMING] rustc_errors test:false 16.949
[00:15:21] [RUSTC-TIMING] syntax test:false 66.716
[00:15:21]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:15:59] [RUSTC-TIMING] syntax_ext test:false 37.624
[00:18:20] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:21:55]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:21:55]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:21:55]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:22:25] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:26:37] [RUSTC-TIMING] rustc_metadata test:false 281.920
[00:27:30]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:27:40] [RUSTC-TIMING] rustc_typeck test:false 345.041
[00:28:45] [RUSTC-TIMING] rustc_lint test:false 75.117
---
[00:31:52]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:31:55] [RUSTC-TIMING] rustc_borrowck test:false 39.844
[00:32:19] [RUSTC-TIMING] rustc_passes test:false 63.547
[00:32:48] [RUSTC-TIMING] rustc_save_analysis test:false 64.203
[00:33:30] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:34:02] [RUSTC-TIMING] rustc_driver test:false 74.066
[00:34:02]    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
[00:34:03] [RUSTC-TIMING] rustc_binary test:false 0.636
[00:34:03]     Finished release [optimized] target(s) in 21m 05s
---
[00:44:50]    Compiling rustc-rayon-core v0.1.1
[00:44:50] [RUSTC-TIMING] stable_deref_trait test:false 0.224
[00:44:50]    Compiling byteorder v1.2.7
[00:44:50]    Compiling either v1.5.0
[00:44:50] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:44:50]    Compiling unicode-width v0.1.5
[00:44:50]    Compiling bitflags v1.0.4
[00:44:51] [RUSTC-TIMING] unicode_width test:false 0.273
[00:44:51]    Compiling graphviz v0.0.0 (/checkout/src/libgraphviz)
---
[00:47:03]    |
[00:47:03] 71 | #[allow_internal_unstable]
[00:47:03]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:47:03] 
[00:47:50] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:49:01]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:49:04] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:49:04]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:71:1
[00:49:04]    |
[00:49:04]    |
[00:49:04] 71 | #[allow_internal_unstable]
[00:49:04]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:49:04] 
[00:49:50] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:50:15] [RUSTC-TIMING] syntax_ext test:false 74.197
[00:53:30] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105001 ms
[00:57:40] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105002 ms
[00:59:25] emulator: ERROR: detected a hanging thread 'QEMU1 main loop'. No response for 105002 ms
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 139.
travis_time:start:050d0a9f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr  6 02:36:54 UTC 2019
---
travis_time:end:07ffd0e0:start=1554518216153355461,finish=1554518216170551275,duration=17195814
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03ae7440
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:004df468
travis_time:start:004df468
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0532824f
$ dmesg | grep -i kill
