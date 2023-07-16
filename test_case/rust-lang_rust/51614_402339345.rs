plain
[00:44:22] ....................................................................................................
[00:44:27] ....................................................................................................
[00:44:32] ....................................................................................................
[00:44:38] ....................................................................................................
[00:44:43] .............................................................................F......................
[00:44:55] ...............i....................................................................................
[00:45:01] ....................................................................................................
[00:45:07] ....................................................................................................
[00:45:14] .........................................................i..........................................
[00:45:14] .........................................................i..........................................
[00:45:16] ..........................................
[00:45:16] failures:
[00:45:16] 
[00:45:16] ---- [ui] ui/macros/bad_hello.rs stdout ----
[00:45:16] diff of stderr:
[00:45:16] 
[00:45:16] 3    |
[00:45:16] 4 LL |     println!(3 + 4); //~ ERROR expected a literal
[00:45:16] 5    |              ^^^^^ help: consider changing this to: `"{}", 3 + 4`
[00:45:16] +    |
[00:45:16] + note: you might be missing a string literal to format with
[00:45:16] +   --> $DIR/bad_hello.rs:12:14
[00:45:16] +    |
[00:45:16] + LL |     println!(3 + 4); //~ ERROR expected a literal
[00:45:16] 6 
[00:45:16] 7 error: aborting due to previous error
[00:45:16] 8 
[00:45:16] 
[00:45:16] 
[00:45:16] 
[00:45:16] The actual stderr differed from the expected stderr.
[00:45:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/bad_hello.stderr
[00:45:16] To update references, rerun the tests and pass the `--bless` flag
[00:45:16] To only update this specific test, also pass `--test-args macros/bad_hello.rs`
[00:45:16] error: 1 errors occurred comparing output.
[00:45:16] status: exit code: 101
[00:45:16] status: exit code: 101
[ity":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider changing this to","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/macros/bad_hello.rs","byte_start":492,"byte_end":497,"line_start":12,"line_end":12,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    println!(3 + 4); //~ ERROR expected a literal","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"\"{}\", 3 + 4","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: expected a literal\n  --> /checkout/src/test/ui/macros/bad_hello.rs:12:14\n   |\nLL |     println!(3 + 4); //~ ERROR expected a literal\n   |              ^^^^^ help: consider changing this to: `\"{}\", 3 + 4`\n   |\nnote: you might be missing a string literal to format with\n  --> /checkout/src/test/ui/macros/bad_hello.rs:12:14\n   |\nLL |     println!(3 + 4); //~ ERROR expected a literal\n   |              ^^^^^\n\n"}
[00:45:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:16] ------------------------------------------
[00:45:16] 
[00:45:16] thread '[ui] ui/macros/bad_hello.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:45:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:16] 
[00:45:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:45:16] 
[00:45:16] 
[00:45:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:16] 
[00:45:16] 
[00:45:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:16] Build completed unsuccessfully in 0:02:07
[00:45:16] Build completed unsuccessfully in 0:02:07
[00:45:16] make: *** [check] Error 1
[00:45:16] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08ee4d68
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
