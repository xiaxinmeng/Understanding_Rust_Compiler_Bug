\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/edition-lint-nested-paths.rs","byte_start":712,"byte_end":717,"line_start":23,"line_end":23,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    crate fn c() {}","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(crate_visibility_modifier)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: `crate` visibility modifier is experimental (see issue #45388)\n  --> /checkout/src/test/ui/rust-2018/edition-lint-nested-paths.rs:23:5\n   |\nLL |     crate fn c() {}\n   |     ^^^^^\n   |\n   = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable\n\n"}
[00:46:26] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:46:26] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
[00:46:26] ------------------------------------------
[00:46:26] 
[00:46:26] thread '[ui] ui/rust-2018/edition-lint-nested-paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:26] 
---
[00:46:26] 
[00:46:26] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:46:26] 
[00:46:26] 
[00:46:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:26] 
[00:46:26] 
[00:46:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:26] Build completed unsuccessfully in 0:03:01
[00:46:26] Build completed unsuccessfully in 0:03:01
[00:46:26] Makefile:58: recipe for target 'check' failed
[00:46:26] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:32051bd8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0029730e:start=1536256129602585411,finish=1536256129608858098,duration=6272687
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1773a854
$ ln -s . checkout && f
