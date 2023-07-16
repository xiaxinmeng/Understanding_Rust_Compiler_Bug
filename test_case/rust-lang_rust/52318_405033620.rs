plain
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:45:59] 
[00:45:59] running 1567 tests
[00:46:02] ..................................................................................................i.
[00:46:06] .............................F..................................i...................................
[00:46:11] ....................................................................................................
[00:46:14] ....................................................................................................
[00:46:16] ....................................................................................................
[00:46:19] ....................................................................................................
---
[00:46:48] 
[00:46:48] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:46:48] 
[00:46:48] 
[00:46:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout1.5G  284K  1.5G   1% /run
none            4.0K     0  4.0K   0% /sys/fs/cgroup
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
---
143820 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
136548 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
136544 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130340 ./obj/build/bootstrap/debug/incremental/bootstrap-3kaq1kqcanyi4
130336 ./obj/build/bootstrap/debug/incremental/bootstrap-3kaq1kqcanyi4/s-f2wf96hpp8-186d7sn-1i9mqtv27979p
129736 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64unknown-linux-gnu/stage0-rustc
53520 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
53516 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
53136 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
