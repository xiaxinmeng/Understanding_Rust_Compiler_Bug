\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0225.rs","byte_start":54,"byte_end":68,"line_start":3,"line_end":3,"column_start":29,"column_end":43,"is_primary":false,"text":[{"text":"trait Foo = std::io::Read + std::io::Write;","highlight_start":29,"highlight_end":43}],"label":"non-auto additional trait","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/error-codes/E0225.rs","byte_start":237,"byte_end":240,"line_start":8,"line_end":8,"column_start":16,"column_end":19,"is_primary":true,"text":[{"text":"    let _: Box<Foo>;","highlight_start":16,"highlight_end":19}],"label":"expanded from this trait alias","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0225]: only auto traits can be used as additional traits in a trait object\n  --> /checkout/src/test/ui/error-codes/E0225.rs:8:16\n   |\nLL | trait Foo = std::io::Read + std::io::Write;\n   |                             -------------- non-auto additional trait\n...\nLL |     let _: Box<Foo>;\n   |                ^^^ expanded from this trait alias\n\n"}
[01:15:55] {"message":"For more information about this error, try `rustc --explain E0225`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0225`.\n"}
[01:15:55] 
[01:15:55] ------------------------------------------
[01:15:55] 
---
[01:15:55] 
[01:15:55] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:15:55] 
[01:15:55] 
[01:15:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:55] 
[01:15:55] 
[01:15:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:55] Build completed unsuccessfully in 0:04:29
[01:15:55] Build completed unsuccessfully in 0:04:29
[01:15:55] make: *** [check] Error 1
[01:15:55] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04079f40
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 28 03:38:18 UTC 2019
