\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/static-not-unpin.rs","byte_start":633,"byte_end":645,"line_start":22,"line_end":22,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    assert_unpin(generator);","highlight_start":5,"highlight_end":17}],"label":"the trait `std::pin::Unpin` is not implemented for `[static generator@/checkout/src/test/ui/generator/static-not-unpin.rs:19:25: 21:6 _]`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `assert_unpin`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/generator/static-not-unpin.rs","byte_start":522,"byte_end":553,"line_start":15,"line_end":15,"column_start":1,"column_end":32,"is_primary":true,"text":[{"text":"fn assert_unpin<T: Unpin>(_: T) {","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/generator/static-not-unpin.rs:19:25: 21:6 _]: std::pin::Unpin` is not satisfied\n  --> /checkout/src/test/ui/generator/static-not-unpin.rs:22:5\n   |\nLL |     assert_unpin(generator);\n   |     ^^^^^^^^^^^^ the trait `std::pin::Unpin` is not implemented for `[static generator@/checkout/src/test/ui/generator/static-not-unpin.rs:19:25: 21:6 _]`\n   |\nnote: required by `assert_unpin`\n  --> /checkout/src/test/ui/generator/static-not-unpin.rs:15:1\n   |\nLL | fn assert_unpin<T: Unpin>(_: T) {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:53:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:53:10] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:53:10] ------------------------------------------
[00:53:10] 
[00:53:10] thread '[ui] ui/generator/static-not-unpin.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:53:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:53:10] 
[00:53:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:53:10] 
[00:53:10] 
[00:53:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:10] 
[00:53:10] 
[00:53:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:10] Build completed unsuccessfully in 0:03:48
[00:53:10] Build completed unsuccessfully in 0:03:48
[00:53:10] Makefile:58: recipe for target 'check' failed
[00:53:10] make: *** [check] Error 1
55844 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
55744 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
55740 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
52788 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps
