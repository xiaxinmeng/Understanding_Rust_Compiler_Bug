\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/static-not-unpin.rs","byte_start":633,"byte_end":645,"line_start":22,"line_end":22,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    assert_unpin(generator);","highlight_start":5,"highlight_end":17}],"label":"the trait `std::pin::Unpin` is not implemented for `[static generator@/checkout/src/test/ui/generator/static-not-unpin.rs:19:25: 21:6 _]`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `assert_unpin`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/generator/static-not-unpin.rs","byte_start":522,"byte_end":553,"line_start":15,"line_end":15,"column_start":1,"column_end":32,"is_primary":true,"text":[{"text":"fn assert_uned; 24 ignored; 0 measured; 0 filtered out
[00:50:02] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:50:02] 
[00:50:02] 
[00:50:02] 
[00:50:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:02] 
[00:50:02] 
[00:50:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:02] Build completed unsuccessfully in 0:03:39
[00:50:02] Build completed unsuccessfully in 0:03:39
[00:50:02] Makefile:58: recipe for target 'check' failed
travis_time:end:28d14aaf:start=1541438213255111125,finish=1541441215701361951,duration=3002446250826

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:008a4ce9
---
travis_time:end:21a2fd98:start=1541441216704983568,finish=1541441216711152187,duration=6168619
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06437460
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'
