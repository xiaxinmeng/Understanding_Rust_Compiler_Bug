plain
[00:38:59] ....................................................................................................
[00:39:04] ....................................................................................................
[00:39:09] ....................................................................................................
[00:39:15] ....................................................................................................
[00:39:19] .............................................................................F......................
[00:39:30] ...............i....................................................................................
[00:39:36] ....................................................................................................
[00:39:41] ....................................................................................................
[00:39:48] .........................................................i..........................................
---
[00:39:50] diff of stderr:
[00:39:50] 
[00:39:50] 2   --> $DIR/bad_hello.rs:12:14
[00:39:50] 3    |
[00:39:50] 4 LL |     println!(3 + 4); //~ ERROR expected a literal
[00:39:50] -    |              ^^^^^ help: consider changing this to: `"{}", 3 + 4`
[00:39:50] +    |              ^^^^^ help: consider changing this to: `prinln!("{}"), 3 + 4`
[00:39:50] 7 error: aborting due to previous error
[00:39:50] 8 
[00:39:50] 
[00:39:50] 
[00:39:50] 
[00:39:50] The actual stderr differed from the expected stderr.
[00:39:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/bad_hello.stderr
[00:39:50] To update references, rerun the tests and pass the `--bless` flag
[00:39:50] To only update this specific test, also pass `--test-args macros/bad_hello.rs`
[00:39:50] error: 1 errors occurred comparing output.
[00:39:50] status: exit code: 101
[00:39:50] status: exit code: 101
[00:39:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/bad_hello.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/auxiliary" "-A" "unused"
[00:39:50] ------------------------------------------
[00:39:50] 
[00:39:50] ------------------------------------------
[00:39:50] stderr:
[00:39:50] stderr:
[00:39:50] ------------------------------------------
[00:39:50] {"message":"expected a literal","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/bad_hello.rs","byte_start":492,"byte_end":497,"line_start":12,"line_end":12,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    println!(3 + 4); //~ ERROR expected a literal","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider changing this to","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/macros/bad_hello.rs","byte_start":492,"byte_end":497,"line_start":12,"line_end":12,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    println!(3 + 4); //~ ERROR expected a literal","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"prinln!(\"{}\"), 3 + 4","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: expected a literal\n  --> /checkout/src/test/ui/macros/bad_hello.rs:12:14\n   |\nLL |     println!(3 + 4); //~ ERROR expected a literal\n   |              ^^^^^ help: consider changing this to: `prinln!(\"{}\"), 3 + 4`\n\n"}
[00:39:50] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:39:50] ------------------------------------------
[00:39:50] 
[00:39:50] thread '[ui] ui/macros/bad_hello.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:39:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:39:50] 
[00:39:50] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:39:50] 
[00:39:50] 
[00:39:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:39:50] 
[00:39:50] 
[00:39:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:39:50] Build completed unsuccessfully in 0:01:57
[00:39:50] Build completed unsuccessfully in 0:01:57
[00:39:50] Makefile:58: recipe for target 'check' failed
[00:39:50] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0aa6e0e8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
