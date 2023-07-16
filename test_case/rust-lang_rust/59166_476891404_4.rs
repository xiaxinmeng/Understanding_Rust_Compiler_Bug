\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-ambiguous.rs","byte_start":290,"byte_end":293,"line_start":21,"line_end":21,"column_start":7,"column_end":10,"is_primary":true,"text":[{"text":"    t.foo(); //~ ERROR E0034","highlight_start":7,"highlight_end":10}],"label":"multiple `foo` found","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"candidate #1 is defined in an impl of the trait `inner::A` for the type `u8`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-ambiguous.rs","byte_start":137,"byte_end":150,"line_start":8,"line_end":8,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"        fn foo(&self) {}","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"candidate #2 is defined in an impl of the trait `inner::B` for the type `u8`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-ambiguous.rs","byte_start":188,"byte_end":201,"line_start":11,"line_end":11,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"        fn foo(&self) {}","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0034]: multiple applicable items in scope\n  --> /checkout/src/test/ui/traits/trait-alias-ambiguous.rs:21:7\n   |\nLL |     t.foo(); //~ ERROR E0034\n   |       ^^^ multiple `foo` found\n   |\nnote: candidate #1 is defined in an impl of the trait `inner::A` for the type `u8`\n  --> /checkout/src/test/ui/traits/trait-alias-ambiguous.rs:8:9\n   |\nLL |         fn foo(&self) {}\n   |         ^^^^^^^^^^^^^\nnote: candidate #2 is defined in an impl of the trait `inner::B` for the type `u8`\n  --> /checkout/src/test/ui/traits/trait-alias-ambiguous.rs:11:9\n   |\nLL |         fn foo(&self) {}\n   |         ^^^^^^^^^^^^^\n\n"}
[01:12:06] {"message":"For more information about this error, try `rustc --explain E0034`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0034`.\n"}
[01:12:06] 
[01:12:06] ------------------------------------------
[01:12:06] 
---
[01:12:06] 
[01:12:06] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:12:06] 
[01:12:06] 
[01:12:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:06] 
[01:12:06] 
[01:12:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:06] Build completed unsuccessfully in 0:04:18
[01:12:06] Build completed unsuccessfully in 0:04:18
[01:12:06] Makefile:48: recipe for target 'check' failed
[01:12:06] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04425fea
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 26 23:18:13 UTC 2019
