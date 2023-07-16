\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-48364.rs","byte_start":27,"byte_end":38,"line_start":2,"line_end":2,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"    b\"\".starts_with(stringify!(foo))","highlight_start":9,"highlight_end":20}],"label":"the trait `std::needle::Needle<&[u8]>` is not implemented for `&str`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the following implementations were found:\n  <&'p str as std::needle::Needle<&'h std::ffi::OsStr>>\n  <&'p str as std::needle::Needle<H>>\n  <&'q &'p str as std::needle::Needle<H>>","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `&str: std::needle::Needle<&[u8]>` is not satisfied\n  --> /checkout/src/test/ui/issues/issue-48364.rs:2:9\n   |\nLL |     b\"\".starts_with(stringify!(foo))\n   |         ^^^^^^^^^^^ the trait `std::needle::Needle<&[u8]>` is not implemented for `&str`\n   |\n   = help: the following implementations were found:\n             <&'p str as std::needle::Needle<&'h std::ffi::OsStr>>\n             <&'p str as std::needle::Needle<H>>\n             <&'q &'p str as std::needle::Needle<H>>\n\n"}
[01:14:19] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[01:14:19] 
[01:14:19] ------------------------------------------
[01:14:19] 
---
[01:14:19] 
[01:14:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:14:19] 
[01:14:19] 
[01:14:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:19] 
[01:14:19] 
[01:14:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:19] Build completed unsuccessfully in 0:04:26
[01:14:19] Build completed unsuccessfully in 0:04:26
[01:14:19] make: *** [check] Error 1
[01:14:19] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07a32527
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr  1 17:18:13 UTC 2019
