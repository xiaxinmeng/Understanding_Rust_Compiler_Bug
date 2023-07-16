\n"},"level":"error","spans":[{"file_name":"<print_me macros>","byte_start":26,"byte_end":34,"line_start":1,"line_end":1,"column_start":27,"column_end":35,"is_primary":true,"text":[{"text":"( $ p : path ) => { { use $ p as V ; println ! ( \"{}\" , V ) ; } }","highlight_start":27,"highlight_end":35}],"label":"no `y` in `x`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/rust-2018/inject-2015-use-root-module.rs","byte_start":960,"byte_end":983,"line_start":28,"line_end":28,"column_start":5,"column_end":28,"is_primary":false,"text":[{"text":"    print_me!(crate::x::y);","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"print_me!","def_site_span":{"file_name":"<print_me macros>","byte_start":0,"byte_end":65,"line_start":1,"line_end":1,"column_start":1,"column_end":66,"is_primary":false,"text":[{"text":"( $ p : path ) => { { use $ p as V ; println ! ( \"{}\" , V ) ; } }","highlight_start":1,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error[E0432]: unresolved import `crate::x::y`\n  --> /checkout/src/test/ui/rust-2018/inject-2015-use-root-module.rs:28:5\n   |\nLL |     print_me!(crate::x::y);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^ no `y` in `x`\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:43:53] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:53] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[00:43:53] ------------------------------------------
[00:43:53] 
[00:43:53] 
[00:43:53] thread '[ui] ui/rust-2018/inject-2015-use-root-module.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3044:9
[00:43:53] 
[00:43:53] failures:
[00:43:53] failures:
[00:43:53]     [ui] ui/rust-2018/inject-2015-use-root-module-path.rs
[00:43:53]     [ui] ui/rust-2018/inject-2015-use-root-module.rs
[00:43:53] test result: FAILED. 1438 passed; 2 failed; 16 ignored; 0 measured; 0 filtered out
[00:43:53] 
[00:43:53] 
[00:43:53] 
[00:43:53] 
[00:43:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:53] 
[00:43:53] 
[00:43:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:53] Build completed unsuccessfully in 0:02:25
[00:43:53] Build completed unsuccessfully in 0:02:25
[00:43:53] make: *** [check] Error 1
[00:43:53] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a7e6e2d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
