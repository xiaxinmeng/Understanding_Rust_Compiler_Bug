plain
Resolving deltas: 100% (612610/612610), completed with 4869 local objects.
---
[00:00:39] configure: rust.quiet-tests     := True
---
[00:44:32] ..........................................................................i.........................
[00:44:38] .................i..................................................................................
---
[00:45:14] ..............................................................................................i.....
[00:45:22] .......................................F.............................i..............................
[00:45:28] ....................................................................................................
[00:45:35] ....................................................................................................
[00:45:43] ....................................................................................................
`&`-reference `fancy_ref` [E0594]"
---
[00:45:44]         msg: "E0594"
---
[00:45:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:44] expected success, got: exit code: 101
[00:45:44]
[00:45:44]
[00:45:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:44] Build completed unsuccessfully in 0:02:29
[00:45:44] Makefile:58: recipe for target 'check' failed
[00:45:44] make: *** [check] Error 1
