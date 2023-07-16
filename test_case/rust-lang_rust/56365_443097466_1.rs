\n\nPaths in `use` statements are relative to the crate root. To import items\nrelative to the current and parent modules, use the `self::` and `super::`\nprefixes, respectively. Also verify that you didn't misspell the import\nname and that the import ex":null}],"children":[],"rendered":"error[E0531]: cannot find unit struct/variant or constant `Self` in this scope\n  --> /checkout/src/test/ui/self/self_type_keyword-2.rs:14:9\n   |\nLL |     let Self = 5;\n   |         ^^^^ not found in this scope\n\n"}
[00:47:56] {"message":"cannot find unit struct/variant or constant `Self` in this scope","code":{"code":"E0531","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/self/self_type_keyword-2.rs","byte_start":670,"byte_end":674,"line_start":18,"line_end":18,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        Self => (),","highlight_start":9,"highlight_end":13}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0531]: cannot find unit struct/variant or constant `Self` in this scope\n  --> /checkout/src/test/ui/self/self_type_keyword-2.rs:18:9\n   |\nLL |         Self => (),\n   |         ^^^^ not found in this scope\n\n"}
[00:47:56] {"message":"cannot find unit struct/variant or constant `Self` in this scope","code":{"code":"E0531","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/self/self_type_keyword-2.rs","byte_start":783,"byte_end":787,"line_start":20,"line_end":20,"column_start":18,"column_end":22,"is_primary":true,"text":[{"text":"        Foo { x: Self } => (),","highlight_start":18,"highlight_end":22}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0531]: cannot find unit struct/variant or constant `Self` in this scope\n  --> /checkout/src/test/ui/self/self_type_keyword-2.rs:20:18\n   |\nLL |         Foo { x: Self } => (),\n   |                  ^^^^ not found in this scope\n\n"}
[00:47:56] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:47:56] {"message":"Some errors occurred: E0432, E0531.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0432, E0531.\n"}
[00:47:56] {"message":"For more information about an error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0432`.\n"}
[00:47:56] ------------------------------------------
[00:47:56] 
[00:47:56] thread '[ui] ui/self/self_type_keyword-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:47:56] 
---
[00:47:56] 
[00:47:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:47:56] 
[00:47:56] 
[00:47:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:56] 
[00:47:56] 
[00:47:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:56] Build completed unsuccessfully in 0:03:41
[00:47:56] Build completed unsuccessfully in 0:03:41
[00:47:56] Makefile:58: recipe for target 'check' failed
[00:47:56] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:33f8fa60
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 30 05:40:13 UTC 2018
