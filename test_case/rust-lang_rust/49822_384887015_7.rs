\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/yield-while-iterating.rs","byte_start":1798,"byte_end":1799,"line_start":67,"line_end":67,"column_start":20,"column_end":21,"is_primary":true,"text":[{"text":"    println!(\"{}\", x[0]); //~ ERROR","highlight_start":20,"highlight_end":21}],"label":"immutable borrow occurs here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/yield-while-iterating.rs","byte_start":1710,"byte_end":1777,"line_start":62,"line_end":66,"column_start":17,"column_end":6,"is_primary":false,"text":[{"text":"    let mut b = || {","highlight_start":17,"highlight_end":21},{"text":"        for p in &mut x {","highlight_start":1,"highlight_end":26},{"text":"            yield p;","highlight_start":1,"highlight_end":21},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":"mutable borrow occurs here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/generator/yield-while-iterating.rs","byte_start":1819,"byte_end":1820,"line_start":68,"line_end":68,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    b.resume();","highlight_start":5,"highlight_end":6}],"label":"borrow later used here","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable\n  --> /checkout/src/test/ui/generator/yield-while-iterating.rs:67:20\n   |\nLL |       let mut b = || {\n   |  _________________-\nLL | |         for p in &mut x {\nLL | |             yield p;\nLL | |         }\nLL | |     };\n   | |_____- mutable borrow occurs here\nLL |       println!(\"{}\", x[0]); //~ ERROR\n   |                      ^ immutable borrow occurs here\nLL |       b.resume();\n   |       - borrow later used here\n\n"}
[00:44:39] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:44:39] {"message":"Some errors occurred: E0502, E0626.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0502, E0626.\n"}
[00:44:39] {"message":"For more information about an error, try `rustc --explain E0502`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0502`.\n"}
[00:44:39] ------------------------------------------
[00:44:39] 
[00:44:39] thread '[ui (nll)] ui/generator/yield-while-iterating.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2963:9
[00:44:39] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:44:39] 
[00:44:39] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:44:39] 
[00:44:39] 
[00:44:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:44:39] 
[00:44:39] 
[00:44:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:39] Build completed unsuccessfully in 0:03:42
[00:44:39] Build completed unsuccessfully in 0:03:42
[00:44:39] Makefile:58: recipe for target 'check' failed
[00:44:39] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bb5397b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
