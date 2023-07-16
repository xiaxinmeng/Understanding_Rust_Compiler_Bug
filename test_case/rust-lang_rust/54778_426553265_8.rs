\n\nLifetime elision in implementation headers was part of the lifetime elision\nRFC. It is, however, [currently unimplemented][iss15872].\n\n[book-le]: https://doc.rust-lang.org/nightly/book/first-edition/lifetimes.html#lifetime-elision\n[iss15872]: https://github.com/rust-lang/rust/issues/15872\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs","byte_start":921,"byte_end":923,"line_start":30,"line_end":30,"column_start":35,"column_end":37,"is_primary":true,"text":[{"text":"fn foo2(_: &'_ u8, y: &'_ u8) -> &'_ u8 { y } //~ ERROR missing lifetime specifier","highlight_start":35,"highlight_end":37}],"label":"expected lifetime parameter","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `_` or `y`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0106]: missing lifetime specifier\n  --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:30:35\n   |\nLL | fn foo2(_: &'_ u8, y: &'_ u8) -> &'_ u8 { y } //~ ERROR missing lifetime specifier\n   |                                   ^^ expected lifetime parameter\n   |\n   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `_` or `y`\n\n"}
[00:51:12] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:51:12] {"message":"Some errors occurred: E0106, E0262.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0106, E0262.\n"}
[00:51:12] {"message":"For more information about an error, try `rustc --explain E0106`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0106`.\n"}
[00:51:12] ------------------------------------------
[00:51:12] 
[00:51:12] thread '[ui] ui/underscore-lifetime/underscore-lifetime-binders.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:51:12] 
---
[00:51:12] 
[00:51:12] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:51:12] 
[00:51:12] 
[00:51:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:12] 
[00:51:12] 
[00:51:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:12] Build completed unsuccessfully in 0:03:35
[00:51:12] Build completed unsuccessfully in 0:03:35
[00:51:12] Makefile:58: recipe for target 'check' failed
[00:51:12] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02f25413
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
