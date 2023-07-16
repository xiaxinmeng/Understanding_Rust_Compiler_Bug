\n\nThis will fail because the compiler does not know which instance of `Foo` to\ncall `bar` on. Change `Foo::bar()` to `Foo::<T>::bar()` to resolve the error.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-51116.rs","byte_start":655,"byte_end":660,"line_start":16,"line_end":16,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"            *tile = 0;","highlight_start":13,"highlight_end":18}],"label":"cannot infer type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-51116.rs","byte_start":563,"byte_end":566,"line_start":14,"line_end":14,"column_start":21,"column_end":24,"is_primary":false,"text":[{"text":"        for tile in row {","highlight_start":21,"highlight_end":24}],"label":"the element type for this iterator is not specified","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/issue-51116.rs","byte_start":563,"byte_end":566,"line_start":14,"line_end":14,"column_start":21,"column_end":24,"is_primary":false,"text":[{"text":"        for tile in row {","highlight_start":21,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"/checkout/src/test/ui/issue-51116.rs","byte_start":563,"byte_end":566,"line_start":14,"line_end":14,"column_start":21,"column_end":24,"is_primary":false,"text":[{"text":"        for tile in row {","highlight_start":21,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"type must be known at this point","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0282]: type annotations needed\n  --> /checkout/src/test/ui/issue-51116.rs:16:13\n   |\nLL |         for tile in row {\n   |                     --- the element type for this iterator is not specified\nLL |             //~^ NOTE the element type for this iterator is not specified\nLL |             *tile = 0;\n   |             ^^^^^ cannot infer type\n   |\n   = note: type must be known at this point\n\n"}
[00:47:59] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:59] {"message":"For more information about this error, try `rustc --explain E0282`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0282`.\n"}
[00:47:59] ------------------------------------------
[00:47:59] 
[00:47:59] thread '[ui] ui/issue-51116.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:47:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:59] 
[00:47:59] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:47:59] 
[00:47:59] 
[00:47:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:59] 
[00:47:59] 
[00:47:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:59] Build completed unsuccessfully in 0:01:49
[00:47:59] Build completed unsuccessfully in 0:01:49
[00:47:59] Makefile:58: recipe for target 'check' failed
[00:47:59] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0567cec4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1414112d:start=1532097782219063882,finish=1532097782232155023,duration=13091141
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b30c14
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0212e486
$ dmesg | grep -i kill
