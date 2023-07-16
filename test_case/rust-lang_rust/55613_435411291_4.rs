\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/trait-import-suggestions.rs","byte_start":1351,"byte_end":1364,"line_start":50,"line_end":50,"column_start":13,"column_end":26,"is_primary":true,"text":[{"text":"    let y = u32::from_str(\"33\");","highlight_start":13,"highlight_end":26}],"label":"function or associated item not found in `u32`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"items from traits can only be used if the trait is in scope","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"the following trait is implemented but not in scope, perhaps add a `use` for it:","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/trait-import-suggestions.rs","byte_start":692,"byte_end":692,"line_start":18,"line_end":18,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod foo {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::str::FromStr;\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0599]: no function or associated item named `from_str` found for type `u32` in the current scope\n  --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:50:13\n   |\nLL |     let y = u32::from_str(\"33\");\n   |             ^^^^^^^^^^^^^ function or associated item not found in `u32`\n   |\n   = help: items from traits can only be used if the trait is in scope\nhelp: the following trait is implemented but not in scope, perhaps add a `use` for it:\n   |\nLL | use std::str::FromStr;\n   |\n\n"}
[00:46:29] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:46:29] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[00:46:29] ------------------------------------------
[00:46:29] 
[00:46:29] thread '[ui] ui/rust-2018/trait-import-suggestions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:46:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:29] 
[00:46:29] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:46:29] 
[00:46:29] 
[00:46:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:29] 
[00:46:29] 
[00:46:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:29] Build completed unsuccessfully in 0:03:30
[00:46:29] Build completed unsuccessfully in 0:03:30
[00:46:29] Makefile:58: recipe for target 'check' failed
[00:46:29] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ea1f766
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:10c91412:start=1541171379785767507,finish=1541171379793358865,duration=7591358
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1621c580
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2489fecb
$ dmesg | grep -i kill
