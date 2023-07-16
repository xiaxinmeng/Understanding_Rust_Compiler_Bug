plain
wn-linux-gnu/test/ui' 'trivial-bounds-lint.rs'
[00:45:25] 
[00:45:25] error: 1 errors occurred comparing output.
[00:45:25] status: exit code: 101
[00:45:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds-lint.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-lint.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds-lint.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:45:25] ------------------------------------------
[00:45:25] 
[00:45:25] ------------------------------------------
[00:45:25] stderr:
[00:45:25] stderr:
[00:45:25] ------------------------------------------
[00:45:25] {"message":"Trait bound i32: std::marker::Copy does not depend on any type or lifetime parameters","code":{"code":"trivial_bounds","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-lint.rs","byte_start":539,"byte_end":564,"line_start":15,"line_end":15,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"struct A where i32: Copy; //~ ERROR","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-lint.rs","byte_start":521,"byte_end":535,"line_start":13,"line_end":13,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"#![deny(trivial_bounds)]","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: Trait bound i32: std::marker::Copy does not depend on any type or lifetime parameters\n  --> /checkout/src/test/ui/trivial-bounds-lint.rs:15:1\n   |\nLL | struct A where i32: Copy; //~ ERROR\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/trivial-bounds-lint.rs:13:9\n   |\nLL | #![deny(trivial_bounds)]\n   |         ^^^^^^^^^^^^^^\n\n"}
[00:45:25] {"message":"Trait bound i32: X<()> does not depend on any type or lifetime parameters","code":{"code":"trivial_bounds","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-lint.rs","byte_start":784,"byte_end":821,"line_start":28,"line_end":28,"column_start":1,"column_end":38,"is_primary":true,"text":[{"text":"fn global_param() where i32: X<()> {} //~ ERROR","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: Trait bound i32: X<()> does not depend on any type or lifetime parameters\n  --> /checkout/src/test/ui/trivial-bounds-lint.rs:28:1\n   |\nLL | fn global_param() where i32: X<()> {} //~ ERROR\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:45:25] {"message":"Trait bound i32: Z does not depend on any type or lifetime parameters","code":{"code":"trivial_bounds","explanationered":"error: aborting due to 4 previous errors\n\n"}
[00:45:25] ------------------------------------------
[00:45:25] 
[00:45:25] thread '[ui] ui/trivial-bounds-lint.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:45:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:25] 
[00:45:25] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:45:25] 
[00:45:25] 
[00:45:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet"
