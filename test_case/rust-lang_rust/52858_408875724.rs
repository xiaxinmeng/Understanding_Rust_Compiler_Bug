plain
[00:55:47] 
[00:55:47] running 247 tests
[00:56:13] .....................i..............................................................................
[00:56:36] ...................i................................................................................
 "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:43] 
[00:56:43] 
[00:56:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:43] Build completed unsuccessfully in 0:15:23
[00:56:43] Build completed unsuccessfully in 0:15:23
[00:56:43] make: *** [check] Error 1
[00:56:43] Makefile:58: recipe for target 'check' failed
2871592 ./obj
2762568 ./obj/build
2165572 ./obj/build/x86_64-unknown-linux-gnu
786328 ./src
---
145460 ./obj/build/bootstrap/debug/incremental
133976 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
133972 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130592 ./obj/build/bootstrap/debug/incremental/bootstrap-c7ee2tfsizs
130588 ./obj/build/bootstrap/debug/incremental/bootstrap-c7ee2tfsizs/s-f3dyez5hwf-3s9z4x-3t5kexjst7huj
128820 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122972 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
121808 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
113404 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
---
travis_time:end:10c3507f:start=1532959607469926352,finish=1532959607477705408,duration=7779056
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0508945e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
