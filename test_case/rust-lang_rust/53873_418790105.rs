plain
[00:48:14] ....................................................................................................
[00:48:17] ....................................................................................................
[00:48:20] ................................i...................................................................
[00:48:23] ....................................................................................................
[00:48:25] .................................................................................iiiiiiiii..........
[00:48:31] ...ii...............................................................................................
[00:48:34] ....................................................................................................
[00:48:37] ..............................................................i.....................................
[00:48:40] ....................................................................................................
---
[00:50:22] ....................................................................................................
[00:50:30] ....................................................................................................
[00:50:39] ....................................................................................................
[00:50:52] ....................................................................................................
[00:51:01] ...................................F................................................................
[00:51:18] ....................................................................................................
[00:51:25] ....................................................................................................
[00:51:33] ....................................................................................................
[00:51:44] .................................................................................i..................
---
[00:54:54] 
[00:54:54] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:54:54] 
[00:54:54] 
[00:54:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:54] 
[00:54:54] 
[00:54:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:54] Build completed unsuccessfully in 0:10:42
[00:54:54] Build completed unsuccessfully in 0:10:42
[00:54:54] Makefile:58: recipe for target 'check' failed
[00:54:54] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cd29820
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:092063f2:start=1536164053546578483,finish=1536164053554604888,duration=8026405
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:133bb5d3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.29561.!checkout!obj!build!x86_64-unknown-linux-gnu!test!run-pass!intrinsic-move-val!a
Source directories searched: /home/travis/build/rust-lang/rust/src:$cdir:$cwd
Reading symbols from obj/build/x86_64-unknown-linux-gnu/test/run-pass/intrinsic-move-val/a...(no debugging symbols found)...done.
[New LWP 29561]
warning: Could not load shared library symbols for 6 libraries, e.g. /lib/x86_64-linux-gnu/libgcc_s.so.1.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/intrinsic-move-val/a'.
Program terminated with signal SIGILL, Illegal instruction.
#0  0x000055ac94e770ba in intrinsic_move_val::main::he550cac715749cad ()
#0  0x000055ac94e770ba in intrinsic_move_val::main::he550cac715749cad ()
#1  0x000055ac94e772f3 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h98cbeff320c1f187 ()
#2  0x00007fdace5ebfe3 in std::panicking::try::do_call::haac499133cc3b53f ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-d36e1a231adc37cd.so
#3  0x00007fdace62df3a in __rust_maybe_catch_panic ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-d36e1a231adc37cd.so
#4  0x00007fdace5ebed8 i
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d30553e
$ dmesg | grep -i kill
