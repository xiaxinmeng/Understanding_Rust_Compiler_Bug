plain
[00:46:12] ....................................................................................................
[00:46:15] ....................................................................................................
[00:46:17] ....................................................................................................
[00:46:20] ....................................................................................................
[00:46:23] .............................................................................................F......
[00:46:28] ..............................................................................i.....................
[00:46:31] ....................................................................................................
[00:46:33] ....................................................................................................
[00:46:37] ....................................................................................................
---
[00:47:06] 
[00:47:06] ---- [ui] ui/issue-11692-1.rs stdout ----
[00:47:06] diff of stderr:
[00:47:06] 
[00:47:06] - error: format argument must be a string literal.
[00:47:06] + error: format argument must be a string literal
[00:47:06] 3    |
[00:47:06] 3    |
[00:47:06] 4 LL |     print!(test!());
[00:47:06] 5    |            ^^^^^^^
[00:47:06] 5    |            ^^^^^^^
[00:47:06] + help: you might be missing a string literal to format with
[00:47:06] +    |
[00:47:06] + LL |     print!("{}", test!());
[00:47:06] 6 
[00:47:06] 7 error: aborting due to previous error
[00:47:06] 8 
[00:47:06] 
[00:47:06] 
[00:47:06] 
[00:47:06] The actual stderr differed from the expected stderr.
[00:47:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-11692-1/issue-11692-1.stderr
[00:47:06] To update references, rerun the tests and pass the `--bless` flag
[00:47:06] To only update this specific test, also pass `--test-args issue-11692-1.rs`
[00:47:06] error: 1 errors occurred comparing output.
[00:47:06] status: exit code: 1
[00:47:06] status: exit code: 1
[00:47:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-11692-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-11692-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-11692-1/auxiliary" "-A" "unused"
[00:47:06] ------------------------------------------
[00:47:06] 
[00:47:06] ------------------------------------------
[00:47:06] stderr:
[00:47:06] stderr:
[00:47:06] ------------------------------------------
[00:47:06] {"message":"format argument must be a string literal","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-11692-1.rs","byte_start":490,"byte_end":497,"line_start":12,"line_end":12,"column_start":12,"column_end":19,"is_primary":true,"text":[{"text":"    print!(test!());","highlight_start":12,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<print macros>","byte_start":54,"byte_end":85,"line_start":2,"line_end":2,"column_start":27,"column_end":58,"is_primary":false,"text":[{"text":"$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":27,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/chec\n"}
[00:47:06] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:06] ------------------------------------------
[00:47:06] 
[00:47:06] thread '[ui] ui/issue-11692-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:47:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:06] test result: FAILED. 2132 passed; 1 failed; 8 ignored; 0 measured; 0 filtered out
[00:47:06] 
[00:47:06] 
[00:47:06] 
[00:47:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:06] 
[00:47:06] 
[00:47:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:06] Build completed unsuccessfully in 0:01:44
[00:47:06] Build completed unsuccessfully in 0:01:44
[00:47:06] Makefile:58: recipe for target 'check' failed
[00:47:06] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2c1edf34
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
