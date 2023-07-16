\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/ch04-00-understanding-ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs","byte_start":10412,"byte_end":10413,"line_start":306,"line_end":306,"column_start":22,"column_end":23,"is_primary":false,"text":[{"text":"                drop(x);","highlight_start":22,"highlight_end":23}],"label":"value moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs","byte_start":10437,"byte_end":10438,"line_start":307,"line_end":307,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"                drop(x); //[ast]~ ERROR use of moved value: `x`","highlight_start":22,"highlight_end":23}],"label":"value used here after move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `x`\n  --> /checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs:307:22\n   |\nLL |                 drop(x);\n   |                      - value moved here\nLL |                 drop(x); //[ast]~ ERROR use of moved value: `x`\n   |                      ^ value used here after move\n   |\n   = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait\n\n"}
[01:13:10] {"message":"aborting due to 32 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 32 previous errors\n\n"}
[01:13:10] {"message":"Some errors occurred: E0382, E0499, E0502, E0503.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0382, E0499, E0502, E0503.\n"}
[01:13:10] 
[01:13:10] ------------------------------------------
[01:13:10] 
[01:13:10] thread '[ui] ui/borrowck/borrowck-describe-lvalue.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
---
[01:13:10] 
[01:13:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:13:10] 
[01:13:10] 
[01:13:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:10] 
[01:13:10] 
[01:13:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:10] Build completed unsuccessfully in 0:04:18
[01:13:10] Build completed unsuccessfully in 0:04:18
[01:13:10] make: *** [check] Error 1
[01:13:10] Makefile:48: recipe for target 'check' failed
2766044 ./obj
2766004 ./obj/build
2093520 ./obj/build/x86_64-unknown-linux-gnu
1371080 ./src
---
147104 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
147100 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
143588 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
142560 ./obj/build/bootstrap/debug/incremental/bootstrap-2vehb6dba088a
142556 ./obj/build/bootstrap/debug/incremental/bootstrap-2vehb6dba088a/s-fasc1m2ar7-552fnu-h9pt7lk04bnw
138372 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
138368 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
135528 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
123640 ./src/llvm-project/llvm/test/CodeGen
