plain
ruby 2.4.1p111 (2017-03-22 revision 58053) [x86_64-linux]
travis_fold:end:system_info

Network availability confirmed.
Setting APT mirror in /etc/apt/sources.list: http://us-central1.gce.archive.ubuntu.com/ubuntu/
Installing APT Packages
travis_time:start:00f146da
$ travis_apt_get_update
travis_time:end:00f146da:start=1536706421224782020,finish=1536706426160793136,duration=4936011116
---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:01] 
[01:24:01] running 191 tests
[01:24:30] ....................................................................................................
[01:25:25] .....................................................F....................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:26:12] failures:
[01:26:12] 
[01:26:12] ---- [run-make] run-make-fulldeps/save-analysis-rfc2126 stdout ----
[01:26:12] 
[01:26:12] 
[01:26:12] error: make failed
[01:26:12] status: exit code: 2
[01:26:12] command: "make"
[01:26:12] stdout:
[01:26:12] ------------------------------------------
[01:26:12] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/save-analysis-rfc2126'
[01:26:12] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126  krate2.rs
[01:26:12] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/save-analysis-rfc2126/save-analysis-rfc2126  extern_absolute_paths.rs -Zsave-analysis --edition=2018
[01:26:12] Makefile:4: recipe for target 'all' failed
[01:26:12] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/save-analysis-rfc2126'
[01:26:12] ------------------------------------------
[01:26:12] stderr:
[01:26:12] ------------------------------------------
[01:26:12] error[E0432]: unresolved import `krate2`
[01:26:12] error[E0432]: unresolved import `krate2`
[01:26:12]   --> extern_absolute_paths.rs:11:5
[01:26:12]    |
[01:26:12] 11 | use krate2::hello;
[01:26:12]    |     ^^^^^^ Could not find `krate2` in `{{root}}`
[01:26:12] 
[01:26:12] error[E0433]: failed to resolve. Could not find `krate2` in `{{root}}`
[01:26:12]   --> extern_absolute_paths.rs:15:7
[01:26:12]    |
[01:26:12] 15 |     ::krate2::hello();
[01:26:12]    |       ^^^^^^ Could not find `krate2` in `{{root}}`
[01:26:12] error: aborting due to 2 previous errors
[01:26:12] 
[01:26:12] Some errors occurred: E0432, E0433.
[01:26:12] For more information about an error, try `rustc --explain E0432`.
[01:26:12] For more information about an error, try `rustc --explain E0432`.
[01:26:12] make[1]: *** [all] Error 1
[01:26:12] ------------------------------------------
[01:26:12] 
[01:26:12] 
[01:26:12] thread '[run-make] run-make-fulldeps/save-analysis-rfc2126' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:26:12] 
[01:26:12] 
[01:26:12] failures:
[01:26:12]     [run-make] run-make-fulldeps/save-analysis-rfc2126
[01:26:12]     [run-make] run-make-fulldeps/save-analysis-rfc2126
[01:26:12] 
[01:26:12] test result: FAILED. 190 pidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:26:12] 
[01:26:12] 
[01:26:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:12] Build completed unsuccessfully in 0:42:31
[01:26:12] Build completed unsuccessfully in 0:42:31
[01:26:12] make: *** [check] Error 1
[01:26:12] Makefile:58: recipe for target 'check' failed
74136 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
74132 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
74128 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
72532 ./src/llvm/lib
