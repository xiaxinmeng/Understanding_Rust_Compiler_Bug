plain
[00:43:24] .............................................................................................i......
[00:43:29] ......................................................................i.............................
[00:43:33] ....................................................................................................
[00:43:39] ....................................................................................................
[00:43:45] ..........................................................................F.........................
[00:43:48] ...i.................iiiiiiiii...................................................
[00:43:48] 
[00:43:48] ---- [ui] ui/suggestions/issue-51244.rs stdout ----
[00:43:48] diff of stderr:
[00:43:48] 
---
[00:43:48] 
[00:43:48] 
[00:43:48] The actual stderr differed from the expected stderr.
[00:43:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/issue-51244.stderr
[00:43:48] To update references, rerun the tests and pass the `--bless` flag
[00:43:48] To only update this specific test, also pass `--test-args suggestions/issue-51244.rs`
[00:43:48] error: 1 errors occurred comparing output.
[00:43:48] status: exit code: 101
[00:43:48] status: exit code: 101
[00:43:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-51244.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/auxiliary" "-A" "unused"
[00:43:48] ------------------------------------------
[00:43:48] 
[00:43:48] ------------------------------------------
[00:43:48] stderr:
[00:43:48] stderr:
[00:43:48] ------------------------------------------
[00:43:48] {"message":"cannot assign to immutable borrowed content `*my_ref`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/suggestions/issue-51244.rs","byte_start":511,"byte_end":522,"line_start":13,"line_end":13,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    *my_ref = 0;","highlight_start":5,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/suggestions/issue-51244.rs","byte_start":487,"byte_end":501,"line_start":12,"line_end":12,"column_start":9,"column_end":23,"is_primary":false,"text":[{"text":"    let ref my_ref @ _ = 0;","highlight_start":9,"highlight_end":23}],"label":"consider changing this to `ref mut my_ref @ _`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*my_ref`\n  --> /checkout/src/test/ui/suggestions/issue-51244.rs:13:5\n   |\nLL |     let ref my_ref @ _ = 0;\n   |         -------------- consider changing this to `ref mut my_ref @ _`\nLL |     *my_ref = 0;\n   |     ^^^^^^^^^^^ cannot borrow as mutable\n\n"}
[00:43:48] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:48] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:43:48] ------------------------------------------
[00:43:48] 
[00:43:48] thread '[ui] ui/suggestions/issue-51244.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:43:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:43:48] 
[00:43:48] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:43:48] 
[00:43:48] 
[00:43:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:48] 
[00:43:48] 
[00:43:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:48] Build completed unsuccessfully in 0:02:32
[00:43:48] Build completed unsuccessfully in 0:02:32
[00:43:48] Makefile:58: recipe for target 'check' failed
[00:43:48] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0077f3cc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
