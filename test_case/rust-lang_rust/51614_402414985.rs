plain
[00:41:21] ....................................................................................................
[00:41:27] ....................................................................................................
[00:41:31] ....................................................................................................
[00:41:37] ....................................................................................................
[00:41:42] .............................................................................F......................
[00:41:53] ...............i....................................................................................
[00:41:59] ....................................................................................................
[00:42:05] ....................................................................................................
[00:42:12] .........................................................i..........................................
---
[00:42:14] diff of stderr:
[00:42:14] 
[00:42:14] 2   --> $DIR/bad_hello.rs:12:14
[00:42:14] 3    |
[00:42:14] 4 LL |     println!(3 + 4); //~ ERROR expected a literal
[00:42:14] -    |              ^^^^^ help: consider changing this to: `"{}", 3 + 4`
[00:42:14] +    |              ^^^^^ help: consider changing this to: `println!("{}", 3 + 4)`
[00:42:14] 7 error: aborting due to previous error
[00:42:14] 8 
[00:42:14] 
[00:42:14] 
[00:42:14] 
[00:42:14] The actual stderr differed from the expected stderr.
[00:42:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/bad_hello.stderr
[00:42:14] To update references, rerun the tests and pass the `--bless` flag
[00:42:14] To only update this specific test, also pass `--test-args macros/bad_hello.rs`
[00:42:14] error: 1 errors occurred comparing output.
[00:42:14] status: exit code: 101
[00:42:14] status: exit code: 101
[00:42:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/bad_hello.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/auxiliary" "-A" "unused"
[00:42:14] ------------------------------------------
[00:42:14] 
[00:42:14] ------------------------------------------
[00:42:14] stderr:
[00:42:14] stderr:
[00:42:14] ------------------------------------------
[00:42:14] {"message":"expected a literal","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/bad_hello.rs","byte_start":492,"byte_end":497,"line_start":12,"line_end":12,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    println!(3 + 4); //~ ERROR expected a literal","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider changing this to","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/macros/bad_hello.rs","byte_start":492,"byte_end":497,"line_start":12,"line_end":12,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    println!(3 + 4); //~ ERROR expected a literal","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"println!(\"{}\", 3 + 4)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: expected a literal\n  --> /checkout/src/test/ui/macros/bad_hello.rs:12:14\n   |\nLL |     println!(3 + 4); //~ ERROR expected a literal\n   |              ^^^^^ help: consider changing this to: `println!(\"{}\", 3 + 4)`\n\n"}
[00:42:14] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:42:14] ------------------------------------------
[00:42:14] 
[00:42:14] thread '[ui] ui/macros/bad_hello.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:42:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:42:14] 
[00:42:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:42:14] 
[00:42:14] 
[00:42:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:42:14] 
[00:42:14] 
[00:42:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:42:14] Build completed unsuccessfully in 0:02:01
[00:42:14] Build completed unsuccessfully in 0:02:01
[00:42:14] make: *** [check] Error 1
[00:42:14] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2a3e74fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
