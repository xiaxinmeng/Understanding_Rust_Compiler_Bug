plain
[00:43:12] ....................................................................................................
[00:43:14] ....................................................................................................
[00:43:17] ....................................................................................................
[00:43:20] ....................................................................................................
[00:43:23] ...................................................................F................................
[00:43:30] ......................................................i.............................................
[00:43:33] ...............................................i....................................................
[00:43:36] ....................................................................................................
[00:43:40] ....................................................................................................
---
[00:43:46] 1 error[E0308]: mismatched types
[00:43:46] -   --> $DIR/issue-52533-1.rs:18:18
[00:43:46] +   --> $DIR/issue-52533-1.rs:19:18
[00:43:46] 3    |
[00:43:46] 4 LL |     gimme(|x, y| y)
[00:43:46] 5    |                  ^ lifetime mismatch
[00:43:46] 6    |
[00:43:46] 6    |
[00:43:46] 7    = note: expected type `&Foo<'_, '_, u32>`
[00:43:46] 8               found type `&Foo<'_, '_, u32>`
[00:43:46] - note: the anonymous lifetime #4 defined on the body at 18:11...
[00:43:46] -   --> $DIR/issue-52533-1.rs:18:11
[00:43:46] + note: the anonymous lifetime #4 defined on the body at 19:11...
[00:43:46] +   --> $DIR/issue-52533-1.rs:19:11
[00:43:46] 11    |
[00:43:46] 12 LL |     gimme(|x, y| y)
[00:43:46] 
[00:43:46] 
[00:43:46] - note: ...does not necessarily outlive the anonymous lifeus lifetime #3 defined on the body at 19:11","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-52533-1.rs","byte_start":719,"byte_end":727,"line_start":19,"line_end":19,"column_start":11,"column_end":19,"is_primary":true,"text":[{"text":"    gimme(|x, y| y)","highlight_start":11,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/issue-52533-1.rs:19:18\n   |\nLL |     gimme(|x, y| y)\n   |                  ^ lifetime mismatch\n   |\n   = note: expected type `&Foo<'_, '_, u32>`\n              found type `&Foo<'_, '_, u32>`\nnote: the anonymous lifetime #4 defined on the body at 19:11...\n  --> /checkout/src/test/ui/issue-52533-1.rs:19:11\n   |\nLL |     gimme(|x, y| y)\n   |           ^^^^^^^^\nnote: ...does not necessarily outlive the anonymous lifetime #3 defined on the body at 19:11\n  --> /checkout/src/test/ui/issue-52533-1.rs:19:11\n   |\nLL |     gimme(|x, y| y)\n   |           ^^^^^^^^\n\n"}
[00:43:46] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:46] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[00:43:46] ------------------------------------------
[00:43:46] 
[00:43:46] thread '[ui] ui/issue-52533-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3141:9
[00:43:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:43:46] 
[00:43:46] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:43:46] 
[00:43:46] 
[00:43:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
travis_time:end:1c5ad088:start=1532613719537317861,finish=1532616346212522631,duration=2626675204770

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21232900
