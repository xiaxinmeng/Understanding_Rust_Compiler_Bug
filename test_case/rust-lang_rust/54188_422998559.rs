plain
[01:08:19] ---- [ui] ui/extern/extern-const.rs stdout ----
[01:08:19] 
[01:08:19] error: failed to compile fixed code
[01:08:19] status: exit code: 1
[01:08:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/extern-const.fixed" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/auxiliary"
[01:08:19] ------------------------------------------
[01:08:19] 
[01:08:19] ------------------------------------------
[01:08:19] stderr:
[01:08:19] stderr:
[01:08:19] ------------------------------------------
[01:08:19] {"message":"linking with `cc` failed: exit code: 1","code":null,"level":"error","spans":[],"children":[{"message":"\"cc\" \"-Wl,--as-needed\" \"-Wl,-z,noexecstack\" \"-m64\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.0.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.1.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.2.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.3.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.4.rcgu.o\" \"-o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a\" \"-Wl,--gc-sections\" \"-pie\" \"-Wl,-zrelro\" \"-Wl,-znow\" \"-nodefaultlibs\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/auxiliary\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,--start-group\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-lstd-e00f764159875a10\" \"-Wl,--end-group\" \"-Wl,-Bstatic\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-26b428365c56e77c.rlib\" \"-Wl,-Bdynamic\" \"-ldl\" \"-lrt\" \"-lpthread\" \"-lpthread\" \"-lgcc_s\" \"-lc\" \"-lm\" \"-lrt\" \"-lpthread\" \"-lutil\" \"-lutil\" \"-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,--enable-new-dtags\"","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.2.rcgu.o: In function `extern_const::main::hbff16dcccc4c621d':\nextern_const.7rcbfp3g-cgu.2:(.text._ZN12extern_const4main17hbff16dcccc4c621dE+0x7): undefined reference to `C'\ncollect2: error: ld returned 1 exit status\n","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: linking with `cc` failed: exit code: 1\n   |\n   = note: \"cc\" \"-Wl,--as-needed\" \"-Wl,-z,noexecstack\" \"-m64\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.0.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.1.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.2.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.3.rcgu.o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.4.rcgu.o\" \"-o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a\" \"-Wl,--gc-sections\" \"-pie\" \"-Wl,-zrelro\" \"-Wl,-znow\" \"-nodefaultlibs\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/auxiliary\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,--start-group\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-lstd-e00f764159875a10\" \"-Wl,--end-group\" \"-Wl,-Bstatic\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-26b428365c56e77c.rlib\" \"-Wl,-Bdynamic\" \"-ldl\" \"-lrt\" \"-lpthread\" \"-lpthread\" \"-lgcc_s\" \"-lc\" \"-lm\" \"-lrt\" \"-lpthread\" \"-lutil\" \"-lutil\" \"-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib\" \"-Wl,--enable-new-dtags\"\n   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.extern_const.7rcbfp3g-cgu.2.rcgu.o: In function `extern_const::main::hbff16dcccc4c621d':\n           extern_const.7rcbfp3g-cgu.2:(.text._ZN12extern_const4main17hbff16dcccc4c621dE+0x7): undefined reference to `C'\n           collect2: error: ld returned 1 exit status\n           \n\n"}
[01:08:19] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:08:19] ------------------------------------------
[01:08:19] 
[01:08:19] thread '[ui] ui/extern/extern-const.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[01:08:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:08:19] 
[01:08:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:08:19] 
[01:08:19] 
[01:08:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:19] 
[01:08:19] 
[01:08:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:19] Build completed unsuccessfully in 0:07:37
[01:08:19] Build completed unsuccessfully in 0:07:37
[01:08:19] make: *** [check] Error 1
[01:08:19] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0548c9ca
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:226da85e:start=1537402445445732105,finish=1537402445453193151,duration=7461046
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:074a11eb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08ab2d2e
travis_time:start:08ab2d2e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23f71198
$ dmesg | grep -i kill
