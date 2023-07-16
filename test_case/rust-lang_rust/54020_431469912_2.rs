\n\nIn this case, `foo` is defined, but is not a struct, so Rust can't use it as\none.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/use-suggestions-extern-prelude.rs","byte_start":546,"byte_end":550,"line_start":15,"line_end":15,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    let _x = Bazz{};","highlight_start":14,"highlight_end":18}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0422]: cannot find struct, variant or union type `Bazz` in this scope\n  --> /checkout/src/test/ui/rust-2018/use-suggestions-extern-prelude.rs:15:14\n   |\nLL |     let _x = Bazz{};\n   |              ^^^^ not found in this scope\n\n"}
[00:49:14] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:14] {"message":"For more information about this error, try `rustc --explain E0422`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0422`.\n"}
[00:49:14] ------------------------------------------
[00:49:14] 
[00:49:14] thread '[ui] ui/rust-2018/use-suggestions-extern-prelude.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3302:9
[00:49:14] 
---
[00:49:14] 
[00:49:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:49:14] 
[00:49:14] 
[00:49:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:14] 
[00:49:14] 
[00:49:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:14] Build completed unsuccessfully in 0:03:33
[00:49:14] Build completed unsuccessfully in 0:03:33
[00:49:14] make: *** [check] Error 1
[00:49:14] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:079aca76
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:38681b80:start=1539976646286932967,finish=1539976646292447204,duration=5514237
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:056d4494
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!chec
