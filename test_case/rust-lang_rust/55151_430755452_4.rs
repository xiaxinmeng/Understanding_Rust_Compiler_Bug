\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/stable/book/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/polonius-smoke-test.rs","byte_start":385,"byte_end":392,"line_start":18,"line_end":18,"column_start":13,"column_end":20,"is_primary":false,"text":[{"text":"    let y = &mut *x;","highlight_start":13,"highlight_end":20}],"label":"borrow of `*x` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/polonius-smoke-test.rs","byte_start":406,"byte_end":407,"line_start":19,"line_end":19,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"    let z = x; //~ ERROR","highlight_start":13,"highlight_end":14}],"labelr a backtrace.
[00:54:08] 
[00:54:08] failures:
[00:54:08]     [ui] ui/nll/polonius-smoke-test.rs
[00:54:08] 
[00:54:08] 
[00:54:08] test result: FAILED. 4601 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out
[00:54:08] 
[00:54:08] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:54:08] 
[00:54:08] 
[00:54:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:08] 
[00:54:08] 
[0
travis_time:end:220c9c90:start=1539800773144546658,finish=1539804021692434916,duration=3248547888258
