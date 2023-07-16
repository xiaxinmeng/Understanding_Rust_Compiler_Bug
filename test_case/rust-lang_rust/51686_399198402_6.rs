\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/book/first-edition/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/issue-51415.rs","byte_start":677,"byte_end":679,"line_start":16,"line_end":16,"column_start":46,"column_end":48,"is_primary":true,"text":[{"text":"    let opt = a.iter().enumerate().find(|(_, &s)| {","highlight_start":46,"highlight_end":48}],"label":"cannot move out of borrowed content","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/issue-51415.rs","byte_start":678,"byte_end":679,"line_start":16,"line_end":16,"column_start":47,"column_end":48,"is_primary":false,"text":[{"text":"    let opt = a.iter().enumerate().find(|(_, &s)| {","highlight_start":47,"highlight_end":48}],"label":"hint: to prevent move, use `ref s` or `ref mut s`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0507]: cannot move out of borrowed content\n  --> /checkout/src/test/ui/borrowck/issue-51415.rs:16:46\n   |\nLL |     let opt = a.iter().enumerate().find(|(_, &s)| {\n   |                                              ^-\n   |                                              ||\n   |                                              |hint: to prevent move, use `ref s` or `ref mut s`\n   |                                              cannot move out of borrowed content\n\n"}
[00:45:46] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:46] {"message":"For more information about this error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0507`.\n"}
[00:45:46] ------------------------------------------
[00:45:46] 
[00:45:46] thread '[ui] ui/borrowck/issue-51415.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:45:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:46] 
[00:45:46] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:45:46] 
[00:45:46] 
[00:45:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:46] 
[00:45:46] 
[00:45:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:46] Build completed unsuccessfully in 0:02:05
[00:45:46] Build completed unsuccessfully in 0:02:05
[00:45:46] Makefile:58: recipe for target 'check' failed
[00:45:46] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:176e6c4f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
