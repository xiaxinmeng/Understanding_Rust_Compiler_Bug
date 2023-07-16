plain
[00:48:17] .................................................................................................... 2300/4660
[00:48:20] .................................................................................................... 2400/4660
[00:48:24] .................................................................................................... 2500/4660
[00:48:29] .................................................................................................... 2600/4660
[00:48:32] .........................................F.......................................................... 2700/4660
[00:48:39] .................................................................................................... 2900/4660
[00:48:42] ...................................................................................i................ 3000/4660
[00:48:45] .................................................................................................... 3100/4660
[00:48:49] ..........................................i.i..ii................................................... 3200/4660
---
[00:49:37] 
[00:49:37] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:49:37] 
[00:49:37] 
[00:49:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "bootstrap-32pr67l4sa8g0
134744 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0/s-f5xk26o26r-m8df0s-25uz5wr4on2bc
129336 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
129332 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122288 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
113444 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
---
travis_time:end:2aeee180:start=1540137373383862253,finish=1540137373389744025,duration=5881772
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:281b3ec0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!check
