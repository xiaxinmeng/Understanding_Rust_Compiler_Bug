\n\nLifetime elision in implementation headers was part of the lifetime elision\nRFC. It is, however, [currently unimplemented][iss15872].\n\n[book-le]: https://doc.rust-lang.org/nightly/book/first-edition/lifetimes.html#lifetime-elision\n[iss15872]: https://github.com/rust-lang/rust/issues/15872\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-26638.rs","byte_start":715,"byte_end":715,"line_start":17,"line_end":17,"column_start":22,"column_end":22,"is_primary":true,"text":[{"text":"fn parse_type_3() -> &str { unimplemented!() }","highlight_start":22,"highlight_end":22}],"label":"expected lifetime parameter","suggested_replacement":null,"expansion":null}],"children":[{"message":"this function's return type contains a borrowed value, but there is no value for it to be borrowed from","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider giving it a 'static lifetime","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0106]: missing lifetime specifier\n  --> /checkout/src/test/ui/issue-26638.rs:17:22\n   |\nLL | fn parse_type_3() -> &str { unimplemented!() }\n   |                      ^ expected lifetime parameter\n   |\n   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from\n   = help: consider giving it a 'static lifetime\n\n"}
[00:45:50] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:45:50] {"message":"For more information about this error, try `rustc --explain E0106`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0106`.\n"}
[00:45:50] ------------------------------------------
[00:45:50] 
[00:45:50] thread '[ui] ui/issue-26638.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3042:9
[00:45:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:50] test result: FAILED. 1428 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[00:45:50] 
[00:45:50] 
[00:45:50] 
[00:45:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:50] 
[00:45:50] 
[00:45:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:50] Build completed unsuccessfully in 0:02:16
[00:45:50] Build completed unsuccessfully in 0:02:16
[00:45:50] make: *** [check] Error 1
[00:45:50] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10ca145f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
