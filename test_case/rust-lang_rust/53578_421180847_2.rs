\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/bastion-of-the-turbofish.rs","byte_start":2175,"byte_end":2192,"line_start":45,"line_end":45,"column_start":27,"column_end":44,"is_primary":true,"text":[{"text":"    let _: (bool, bool) = (oh<woe, is>(me));","highlight_start":27,"highlight_end":44}],"label":"not a function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/bastion-of-the-turbofish.rs","byte_start":2083,"byte_end":2085,"line_start":44,"line_end":44,"column_start":10,"column_end":12,"is_primary":false,"text":[{"text":"    let (oh, woe, is, me) = (\"the\", \"Turbofish\", \"remains\", \"undefeated\");","highlight_start":10,"highlight_end":12}],"label":"`&str` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0618]: expected function, found `&str`\n  --> /checkout/src/test/ui/bastion-of-the-turbofish.rs:45:27\n   |\nLL |     let (oh, woe, is, me) = (\"the\", \"Turbofish\", \"remains\", \"undefeated\");\n   |          -- `&str` defined here\nLL |     let _: (bool, bool) = (oh<woe, is>(me));\n   |                           ^^^^^^^^^^^^^^^^^ not a function\n\n"}
[00:51:58] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:51:58] {"message":"Some errors occurred: E0109, E0573, E0618.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0109, E0573, E0618.\n"}
[00:51:58] {"message":"For more information about an error, try `rustc --explain E0109`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0109`.\n"}
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] thread '[ui] ui/bastion-of-the-turbofish.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:51:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:58] 
[00:51:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[00:51:58] 
[00:51:58] 
[00:51:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:58] 
[00:51:58] 
[00:51:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:58] Build completed unsuccessfully in 0:07:27
[00:51:58] Build completed unsuccessfully in 0:07:27
[00:51:58] make: *** [check] Error 1
[00:51:58] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:26711d4b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
151412 ./src/tools/clang
149128 ./src/llvm-emscripten/test
149020 ./obj/build/bootstrap/debug/incremental
134588 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl
134584 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl/s-f4rzp9c2cl-afh1ys-1jtu7ee7y4raz
130988 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
129860 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
129856 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
128068 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
---
travis_time:end:074f927d:start=1536880274071224279,finish=1536880274075993660,duration=4769381
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06df3d04
$ ln -s . checkout && fo
