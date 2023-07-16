plain
[00:59:29] 
[00:59:29] failures:
[00:59:29] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[00:59:29] 
[00:59:29] ---- [run-make] run-make/override-aliased-flags stdout ----
[00:59:29] 
[00:59:29] error: make failed
[00:59:29] status: exit code: 2
[00:59:29] command: "make"
[00:59:29] command: "make"
[00:59:29] stdout:
[00:59:29] ------------------------------------------
[00:59:29] # Test that `-O` and `-C opt-level` can be specified multiple times.
[00:59:29] # The rightmost flag will be used over any previous flags.
[00:59:29] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags  -Clinker=arm-none-eabi-gcc -O -O main.rs
[00:59:29] Makefile:8: recipe for target 'all' failed
[00:59:29] ------------------------------------------
[00:59:29] stderr:
[00:59:29] ------------------------------------------
[00:59:29] error: linking with `arm-none-eabi-gcc` failed: exit code: 1
[00:59:29] error: linking with `arm-none-eabi-gcc` failed: exit code: 1
[00:59:29]   |
[00:59:29]   = note: "arm-none-eabi-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags/main.main.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags/main.main.7rcbfp3g-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags/main" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags/main.4s37gsrti678ik8u.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-36963fe6a2961b22.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-bad302bb9c2be352.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-068d4c647d225b00.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-dd618d3a61d2285d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-08bd494bdfbe6ec5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-f434e5c003b087ed.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-4ad3589c043e8a63.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-b93458722dc1ee7d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-cbc983130e35415c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-b986af5e424776e6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-08dafd64384d2a0a.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-bc40ea2e2336c57a.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
[00:59:29]           
[00:59:29] 
[00:59:29] error: aborting due to previous error
[00:59:29] 
---
[00:59:29] test result: FAILED. 3 passed; 1 failed; 9 ignored; 0 measured; 0 filtered out
[00:59:29] 
[00:59:29] 
[00:59:29] 
[00:59:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0-rust-1.36.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:29] 
[00:59:29] 
[00:59:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
[00:59:29] Build completed unsuccessfully in 0:54:11
---
travis_time:end:0d3cd01c:start=1556912403722990977,finish=1556912403738983640,duration=15992663
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d04f0ca
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06754404
travis_time:start:06754404
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:010cb2c4
$ dmesg | grep -i kill
