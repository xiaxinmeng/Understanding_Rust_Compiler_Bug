plain
Requirement already satisfied: PyYAML<=3.12,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli)
Collecting botocore==1.10.28 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2e/91/f0870d4de8eb78897ce781f3ff22fc492bbb9849b5c91f26db20b125ef36/botocore-1.10.28-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 45.7MB/s eta 0:00:01
    0% |▏                               | 20kB 39.5MB/s eta 0:00:01
    0% |▎                               | 30kB 44.9MB/s eta 0:00:01
    0% |▎                               | 40kB 16.9MB/s eta 0:00:01
---
[00:50:40] ....................................................................................................
[00:50:43] ....................................................................................................
[00:50:46] ....................................................................................................
[00:50:49] ....................................................................................................
[00:50:54] ...................................................................................F................
[00:51:03] ....................................................................................................
[00:51:09] ................................................................................i...................
[00:51:14] .........................................................i..........................................
[00:51:19] ..............................................................................ii....................
[00:51:19] ..............................................................................ii....................
[00:51:24] ....................................................................................................
[00:51:31] .......................................................................................i............
[00:51:33] .....iiiiiiiii...................................................
[00:51:33] 
[00:51:33] ---- [ui] ui/in-band-lifetimes/impl/trait-underscore.rs stdout ----
[00:51:33] 
[00:51:33] 
[00:51:33] error: test compilation failed although it shouldn't!
[00:51:33] status: exit code: 101
[00:51:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/in-band-lifetimes/impl/trait-underscore.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/impl/trait-underscore/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/impl/trait-underscore/auxiliary" "-A" "unused"
[00:51:33] stdout:
[00:51:33] --------n\n"}
[00:51:33] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:33] {"message":"For more information about this error, try `rustc --explain E0263`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0263`.\n"}
[00:51:33] ------------------------------------------
[00:51:33] 
[00:51:33] thread '[ui] ui/in-band-lifetimes/impl/trait-underscore.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:51:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:33] 
[00:51:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:51:33] 
[00:51:33] 
[00:51:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:33] 
[00:51:33] 
[00:51:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:33] Build completed unsuccessfully in 0:02:49
[00:51:33] Build completed unsuccessfully in 0:02:49
[00:51:33] make: *** [check] Error 1
[00:51:33] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15d520e5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
86_64-unknown-linux-gnu
104164 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
103608 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
103236 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103232 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt/s-f1dqd6elmq-29dsb3-17uifkj4ft9sj
92408 ./obj/build/x86_64-unknown-linux-gnu/stage1
92384 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
90820 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
89812 ./src/llvm/test/CodeGen
