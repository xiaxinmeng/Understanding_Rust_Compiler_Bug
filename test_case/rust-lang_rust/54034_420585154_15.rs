\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/book/first-edition/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm.rs","byte_start":188,"byte_end":189,"line_start":9,"line_end":9,"column_start":30,"column_end":31,"is_primary":true,"text":[{"text":"        A { a: v } if { drop(v); true } => v,","highlight_start":30,"highlight_end":31}],"label":"cannot move out of borrowed content","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0507]: cannot move out of borrowed content\n  --> /checkout/src/test/ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm.rs:9:30\n   |\nLL |         A { a: v } if { drop(v); true } => v,\n   |                              ^ cannot move out of borrowed content\n\n"}
[00:51:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:16] {"message":"For more information about this error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0507`.\n"}
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] 
[00:51:16] thread '[ui] ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:51:16] 
[00:51:16] failures:
[00:51:16]     [ui] ui/bind-by-move/bind-by-move-no-guards.rs
[00:51:16]     [ui] ui/bind-by-move/bind-by-move-no-guards.rs
[00:51:16]     [ui] ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.rs#gate_and_2015
[00:51:16]     [ui] ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.rs#gate_and_2018
[00:51:16]     [ui] ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.rs#gate_and_feature_nll
[00:51:16]     [ui] ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.rs#gate_and_znll
[00:51:16]     [ui] ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.rs#no_gate
[00:51:16]     [ui] ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-across-arms.rs
[00:51:16]     [ui] ui/rfc-0107-bind-by-move-pattern-guards/rfc-reject-double-move-in-first-arm.rs
[00:51:16] test result: FAILED. 6746 passed; 8 failed; 28 ignored; 0 measured; 0 filtered out
[00:51:16] 
[00:51:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:51:16] 
[00:51:16] 
[00:51:16] 
[00:51:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:16] 
[00:51:16] 
[00:51:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:16] Build completed unsuccessfully in 0:07:27
[00:51:16] Build completed unsuccessfully in 0:07:27
[00:51:16] make: *** [check] Error 1
[00:51:16] Makefile:58: recipe for target 'check' failed
2463392 ./obj
2437636 ./obj/build
1821124 ./obj/build/x86_64-unknown-linux-gnu
1067348 ./src
---
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
149020 ./obj/build/bootstrap/debug/incremental
134588 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl
134584 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl/s-f4q9yu9yzu-11o24us-1jtu7ee7y4raz
130972 ./obj/-linux-gnu/lib
72532 ./src/llvm/lib
70532 ./obj/build/x86_64-unknown-linux-gnu/native
70300 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
---
travis_time:end:1630341c:start=1536745862055507935,finish=1536745862061511427,duration=6003492
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dc98c3d
$ ln -s . checkout && f
