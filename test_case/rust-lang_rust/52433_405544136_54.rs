\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-38954.rs","byte_start":489,"byte_end":489,"line_start":11,"line_end":11,"column_start":23,"column_end":23,"is_primary":true,"text":[{"text":"fn _test(ref _p: str) {}","highlight_start":23,"highlight_end":23}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `str`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `str` cannot be known at compilation time\n  --> /checkout/src/test/ui/issue-38954.rs:11:23\n   |\nLL | fn _test(ref _p: str) {}\n   |                       ^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `str`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized>\n\n"}
[00:43:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:30] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:43:30] ------------------------------------------
[00:43:30] 
---
[00:43:30] test result: FAILED. 2095 passed; 18 failed; 6 ignored; 0 measured; 0 filtered out
[00:43:30] 
[00:43:30] 
[00:43:30] 
[00:43:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:30] 
[00:43:30] 
[00:43:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:30] Build completed unsuccessfully in 0:01:39
[00:43:30] Build completed unsuccessfully in 0:01:39
[00:43:30] make: *** [check] Error 1
[00:43:30] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00c0b430
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
