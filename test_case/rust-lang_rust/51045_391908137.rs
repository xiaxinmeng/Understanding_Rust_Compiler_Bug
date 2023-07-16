plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/03/9d/a0b73320e4b9d776b0b68a67c7dbdc4115eb9eceff992d6b56222ba550d3/awscli-1.15.27-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 32.8MB/s eta 0:00:01
    1% |▌                               | 20kB 2.0MB/s eta 0:00:01
    2% |▉                               | 30kB 2.3MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 552kB 1.9MB/s 
Collecting botocore==1.10.27 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/0a/fc/5dbeb052f4b70346ad3eb1ac291d503c0a28ed7e0f806f7337bbb73e3b76/botocore-1.10.27-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 31.9MB/s eta 0:00:01
    0% |▏                               | 20kB 33.6MB/s eta 0:00:01
    0% |▎                               | 30kB 41.1MB/s eta 0:00:01
    0% |▎                               | 40kB 9.1MB/s eta 0:00:01
---
[00:44:48] ..............................i.....................................................................
[00:45:02] ...............................................................i....................................
[00:45:19] ................................................i...................................................
[00:45:43] ....................................................................................................
[00:46:06] ........................................F...........................................................
[00:46:51] ......i.............................................................................................
[00:47:20] ...i.......................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:47:25] .........
[00:47:56] ....................................................................................................
[00:47:56] ....................................................................................................
[00:48:26] .......................................................................ii...........................
[00:49:18] ..................................i....................................................i.ii........test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:49:57] ...............................................................................................iiiii
[00:50:25] ii..................................................................................................
[00:50:55] ....................................................................................................
[00:51:19] ....................................................................................................
---
[00:51:37] 
[00:51:37] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:478:22
[00:51:37] 
[00:51:37] 
[00:51:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:37] 
[00:51:37] 
[00:51:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:37] Build completed unsuccessfully in 0:13:08
[00:51:37] Build completed unsuccessfully in 0:13:08
[00:51:37] make: *** [check] Error 1
[00:51:37] Makefile:58: recipe for target 'check' failed
/build/bootstrap/debug/incremental/bootstrap-2v9vu9lcw5vj2/s-f1cktqcx64-7a0j4a-1f80f2tgi9kxx
102796 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-q44a82dol543
102796 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-q44a82dol543
102792 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-q44a82dol543/s-f1clru1s0d-1x4l8ld-27mgve33e1jja
87728 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental
87064 ./obj/build/x86_64-unknown-linux-gnu/stage1
87040 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
84816 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
