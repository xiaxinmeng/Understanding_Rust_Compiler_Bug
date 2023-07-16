\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-49851/compiler-builtins-error.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"//~ ERROR 1:1: 1:1: can't find crate for `core` [E0463]","highlight_start":1,"highlight_end":1}],"label":"can't find crate","suggested_replacement":null,"expansion":null}],"children":[{"message":"the `thumbv7em-none-eabihf` target may not be installed","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0463]: can't find crate for `core`\n   |\n   = note: the `thumbv7em-none-eabihf` target may not be installed\n\n"}
[00:50:13] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:13] {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
[00:50:13] ------------------------------------------
[00:50:13] 
[00:50:13] thread '[ui] ui/issue-49851/compiler-builtins-error.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3014:9
[00:50:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:13] test result: FAILED. 1418 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[00:50:13] 
[00:50:13] 
[00:50:13] 
[00:50:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:13] 
[00:50:13] 
[00:50:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:13] Build completed unsuccessfully in 0:02:27
[00:50:13] Build completed unsuccessfully in 0:02:27
[00:50:13] make: *** [check] Error 1
[00:50:13] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10fc191e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
