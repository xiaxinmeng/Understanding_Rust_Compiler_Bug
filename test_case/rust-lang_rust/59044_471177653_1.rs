\n\nPlease be sure that a file corresponding to the module exists. If you\nwant to use a module named `file_that_doesnt_exist`, you need to have a file\nnamed `file_that_doesnt_exist.rs` or `file_that_doesnt_exist/mod.rs` in the\nsame directory.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/mod_file_not_exist.rs","byte_start":23,"byte_end":38,"line_start":3,"line_end":3,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"name the file either not_a_real_file.rs or not_a_real_file/mod.rs inside the directory \"/checkout/src/test/ui/parser\"","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0583]: file not found for module `not_a_real_file`\n  --> /checkout/src/test/ui/parser/mod_file_not_exist.rs:3:5\n   |\nLL | mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`\n   |     ^^^^^^^^^^^^^^^\n   |\n   = help: name the file either not_a_real_file.rs or not_a_real_file/mod.rs inside the directory \"/checkout/src/test/ui/parser\"\n\n"}
[01:17:52] {"message":"For more information about this error, try `rustc --explain E0583`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0583`.\n"}
[01:17:52] 
[01:17:52] ------------------------------------------
[01:17:52] 
---
[01:17:52] 
[01:17:52] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:17:52] 
[01:17:52] 
[01:17:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:52] 
[01:17:52] 
[01:17:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:52] Build completed unsuccessfully in 0:04:49
[01:17:52] Build completed unsuccessfully in 0:04:49
[01:17:52] make: *** [check] Error 1
[01:17:52] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2419c87a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  9 13:37:28 UTC 2019
