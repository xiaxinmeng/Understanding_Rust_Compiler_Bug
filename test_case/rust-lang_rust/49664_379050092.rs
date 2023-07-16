plain
127.0.1.1 travis-job-f81fa9c2-a8a9-46b6-bdb4-9c37d188493b travis-job-f81fa9c2-a8a9-46b6-bdb4-9c37d188493b ip4-loopback trusty64
W: Failed to fetch https://packagecloud.io/github/git-lfs/ubuntu/dists/trusty/InRelease  Operation too slow. Less than 10 bytes/sec transferred the last 10 seconds
---
[00:03:35] configure: rust.quiet-tests     := True
---
[01:04:43] ..........................................................................i.........................
[01:04:50] .................i..................................................................................
---
[01:05:33] ...........................................................................................i........
[01:05:41] .................................................................i..................................
---
[01:06:52] .............................................i......................................................
---
[01:11:55] .............................i......................................................................
[01:12:13] ..............................................................i.....................................
[01:12:32] ...............................................i....................................................
[01:12:57] ....................................................................................................
[01:13:24] ....................................................................................................
[01:13:51] ....................................................................................................
[01:14:22] .i...............................................................................................i..
[01:14:41] .........................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[01:15:10] ...........................................................
[01:15:47] ....................................................................................................
[01:16:35] .............................................................ii.....................................
[01:17:13] ........................i....F.....................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:17:36] ..........................i.ii...................
[01:18:27] .....................................................................................iiiiiii........
---
[01:21:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/simd-target-feature-mixup.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-target-feature-mixup.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-target-feature-mixup.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:21:16] error[E0658]: the target feature `avx512bw` is currently unstable
[01:21:16]    --> /checkout/src/test/run-pass/simd-target-feature-mixup.rs:166:22
[01:21:16]     |
[01:21:16] 166 |     #[target_feature(enable = "avx512bw")]
[01:21:16]     |                      ^^^^^^^^^^^^^^^^^^^
[01:21:16]     |
[01:21:16]     = help: add #![feature(avx512_target_feature)] to the crate attributes to enable
[01:21:16]
[01:21:16] error[E0658]: the target feature `avx512bw` is currently unstable
[01:21:16]    --> /checkout/src/test/run-pass/simd-target-feature-mixup.rs:172:22
[01:21:16]     |
[01:21:16] 172 |     #[target_feature(enable = "avx512bw")]
[01:21:16]     |                      ^^^^^^^^^^^^^^^^^^^
[01:21:16]     |
[01:21:16]     = help: add #![feature(avx512_target_feature)] to the crate attributes to enable
[01:21:16]
[01:21:16] error[E0658]: the target feature `avx512bw` is currently unstable
[01:21:16]    --> /checkout/src/test/run-pass/simd-target-feature-mixup.rs:178:22
[01:21:16]     |
[01:21:16] 178 |     #[target_feature(enable = "avx512bw")]
[01:21:16]     |                      ^^^^^^^^^^^^^^^^^^^
[01:21:16]     |
[01:21:16]     = help: add #![feature(avx512_target_feature)] to the crate attributes to enable
[01:21:16]
[01:21:16] error[E0658]: the target feature `avx512bw` is currently unstable
[01:21:16]    --> /checkout/src/test/run-pass/simd-target-feature-mixup.rs:125:26
[01:21:16]     |
[01:21:16] 125 |         #[target_feature(enable = "avx512bw")]
[01:21:16]     |                          ^^^^^^^^^^^^^^^^^^^
[01:21:16]     |
[01:21:16]     = help: add #![feature(avx512_target_feature)] to the crate attributes to enable
---
[01:21:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:16] expected success, got: exit code: 101
[01:21:16]
[01:21:16]
[01:21:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:16] Build completed unsuccessfully in 0:18:09
[01:21:16] make: *** [check] Error 1
[01:21:16] Makefile:58: recipe for target 'check' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:24966c24:start=1522956496673216070,finish=1522956496679970874,duration=6754804
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:03540e70
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.cras
