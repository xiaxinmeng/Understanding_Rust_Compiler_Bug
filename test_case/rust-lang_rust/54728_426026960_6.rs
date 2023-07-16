\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/three-equals.rs","byte_start":1403,"byte_end":1417,"line_start":42,"line_end":42,"column_start":20,"column_end":34,"is_primary":true,"text":[{"text":"        return Err(Span::def_site()","highlight_start":20,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(proc_macro_def_site)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'proc_macro_def_site' (see issue #54724)\n  --> /checkout/src/test/ui-fulldeps/proc-macro/auxiliary/three-equals.rs:42:20\n   |\nLL |         return Err(Span::def_site()\n   |                    ^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(proc_macro_def_site)] to the crate attributes to enable\n\n"}
[01:06:51] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:06:51] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] thread '[ui] ui-fulldeps/proc-macro/three-equals.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[01:06:51] 
---
[01:06:51] 
[01:06:51] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[01:06:51] 
[01:06:51] 
[01:06:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:51] 
[01:06:51] 
[01:06:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:51] Build completed unsuccessfully in 0:19:46
[01:06:51] Build completed unsuccessfully in 0:19:46
[01:06:51] Makefile:58: recipe for target 'check' failed
[01:06:51] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1687cd8b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
