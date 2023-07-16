\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/compile-fail/borrowck/two-phase-reservation-sharing-interference.rs","byte_start":1895,"byte_end":1911,"line_start":46,"line_end":46,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        delay = &mut vec;","highlight_start":9,"highlight_end":25}],"label":"mutable borrow occurs here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/compile-fail/borrowck/two-phase-reservation-sharing-interference.rs","byte_start":1718,"byte_end":1722,"line_start":41,"line_end":41,"column_start":22,"column_end":26,"is_primary":false,"text":[{"text":"        let shared = &vec;","highlight_start":22,"highlight_end":26}],"label":"immutable borrow occurs here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/compile-fail/borrowck/two-phase-reservation-sharing-interference.rs","byte_start":2130,"byte_end":2136,"line_start":50,"line_end":50,"column_start":9,"column_end":15,"is_primary":false,"text":[{"text":"        shared[0];","highlight_start":9,"highlight_end":15}],"label":"borrow later used here","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `vec` as mutable because it is also borrowed as immutable\n  --> /checkout/src/test/compile-fail/borrowck/two-phase-reservation-sharing-interference.rs:46:9\n   |\nLL |         let shared = &vec;\n   |                      ---- immutable borrow occurs here\n...\nLL |         delay = &mut vec;\n   |         ^^^^^^^^^^^^^^^^ mutable borrow occurs here\n...\nLL |         shared[0];\n   |         ------ borrow later used here\n\n"}
[01:01:58] thread 'main' panicked at 'assertion failed: match borrow.kind {{
[01:01:58]     BorrowKind::Shared => false,
[01:01:58]     BorrowKind::Unique | BorrowKind::Mut {{ .. }} => true,
[01:01:58] }}', librustc_mir/borrow_check/mod.rs:1252:13
[01:01:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:58] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:01:58] {"message":"For more information about this error, try `rustc --explain E0502`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0502`.\n"}
[01:01:58] ------------------------------------------
[01:01:58] 
[01:01:58] thread '[compile-fail] compile-fail/borrowck/two-phase-reservation-sharing-interference.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2938:9
[01:01:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:01:58] 
[01:01:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:01:58] 
[01:01:58] 
[01:01:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:58] 
[01:01:58] 
[01:01:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:58] Build completed unsuccessfully in 0:18:24
[01:01:58] Build completed unsuccessfully in 0:18:24
[01:01:58] Makefile:58: recipe for target 'check' failed
[01:01:58] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f81ed45
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
