plain
[00:46:56] ..........................................................................i.........................
[00:47:01] ....................................................................................................
[00:47:06] ....................................................................................................
[00:47:12] ....................................................................................................
[00:47:16] ......i.............F....iiiiiiiii...................................................
[00:47:16] 
[00:47:16] ---- [ui] ui/trait-object-auto-dedup-in-impl.rs stdout ----
[00:47:16] diff of stderr:
[00:47:16] 
[00:47:16] 
[00:47:16] 1 error[E0592]: duplicate definitions with name `test`
[00:47:16] -   --> $DIR/trait-object-auto-dedup-in-impl.rs:21:5
[00:47:16] +   --> $DIR/trait-object-auto-dedup-in-impl.rs:24:5
[00:47:16] 3    |
[00:47:16] 4 LL |     fn test(&self) { println!("one"); } //~ ERROR duplicate definitions with name `test`
[00:47:16] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `test`
[00:47:16] 
[00:47:16] The actual stderr differed from the expected stderr.
[00:47:16] The actual stderr differed from the expected stderr.
[00:47:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-object-auto-dedup-in-impl/trait-object-auto-dedup-in-impl.stderr
[00:47:16] To update references, rerun the tests and pass the `--bless` flag
[00:47:16] To only update this specific test, also pass `--test-args trait-object-auto-dedup-in-impl.rs`
[00:47:16] error: 1 errors occurred comparing output.
[00:47:16] status: exit code: 101
[00:47:16] status: exit code: 101
[00:47:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-object-auto-dedup-in-impl.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-object-auto-dedup-in-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-object-auto-dedup-in-impl/auxiliary" "-A" "unused"
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] ------------------------------------------
[00:47:16] stderr:
[00:47:16] stderr:
[00:47:16] ------------------------------------------
[00:47:16] {"message":"duplicate definitions with name `test`","code":{"code":"E0592","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trait-object-auto-dedup-in-impl.rs","byte_start":738,"byte_end":773,"line_start":24,"line_end":24,"column_start":5,"column_end":40,"is_primary":true,"text":[{"text":"    fn test(&self) { println!(\"one\"); } //~ ERROR duplicate definitions with name `test`","highlight_start":5,"highlight_end":40}],"label":"duplicate definitions for `test`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/trait-object-auto-dedup-in-impl.rs","byte_start":857,"byte_end":892,"line_start":28,"line_end":28,"column_start":5,"column_end":40,"is_primary":false,"text":[{"text":"    fn test(&self) { println!(\"two\"); }","highlight_start":5,"highlight_end":40}],"label":"other definition for `test`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0592]: duplicate definitions with name `test`\n  --> /checkout/src/test/ui/trait-object-auto-dedup-in-impl.rs:24:5\n   |\nLL |     fn test(&self) { println!(\"one\"); } //~ ERROR duplicate definitions with name `test`\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `test`\n...\nLL |     fn test(&self) { println!(\"two\"); }\n   |     ----------------------------------- other definition for `test`\n\n"}
[00:47:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:16] {"message":"For more information about this error, try `rustc --explain E0592`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0592`.\n"}
[00:47:16] ------------------------------------------
[00:47:16] 
[00:47:16] thread '[ui] ui/trait-object-auto-dedup-in-impl.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:47:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:16] 
[00:47:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:47:16] 
[00:47:16] 
[00:47:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:16] 
[00:47:16] 
[00:47:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:16] Build completed unsuccessfully in 0:02:33
[00:47:16] Build completed unsuccessfully in 0:02:33
[00:47:16] Makefile:58: recipe for target 'check' failed
[00:47:16] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2af75ab8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
