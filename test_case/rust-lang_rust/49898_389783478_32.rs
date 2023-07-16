\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-leak.rs","byte_start":891,"byte_end":900,"line_start":35,"line_end":35,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    Foo::test(&4i32); //~ ERROR","highlight_start":5,"highlight_end":14}],"label":"the trait `Foo` is not implemented for `i32`","suggested_replacement":null,"expansion":null}],"children":[{"message":"required by `Foo::test`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/trivgeneric_function(5i32); //~ ERROR","highlight_start":5,"highlight_end":21}],"label":"the trait `Foo` is not implemented for `i32`","suggested_replacement":null,"expansion":null}],"children":[{"message":"required by `generic_function`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-leak.rs","byte_start":960,"byte_end":993,"line_start":39,"line_end":39,"column_start":1,"column_end":34,"is_primary":true,"text":[{"text":"fn generic_function<T: Foo>(t: T) {}","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `i32: Foo` is not satisfied\n  --> /checkout/src/test/ui/trivial-bounds-leak.rs:36:5: in fn foo\n   |\nLL |     generic_function(5i32); //~ ERROR\n   |     ^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `i32`\n   |\nnote: required by `generic_function`\n  --> /checkout/src/test/ui/trivial-bounds-leak.rs:39:1\n   |\nLL | fn generic_function<T: Foo>(t: T) {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:37] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:50:37] {"message":"Some errors occurred: E0277, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0277, E0599.\n"}
[00:50:37] {"message":"For more information about an error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0277`.\n"}
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[ui] ui/trivial-bounds-leak.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:50:37] 
---
[00:50:37] 
[00:50:37] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:50:37] 
[00:50:37] 
[00:50:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:37] 
[00:50:37] 
[00:50:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:37] Build completed unsuccessfully in 0:02:35
[00:50:37] Build completed unsuccessfully in 0:02:35
[00:50:37] make: *** [check] Error 1
[00:50:37] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e5fa80c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
