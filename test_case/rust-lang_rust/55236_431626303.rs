plain
[00:47:46] .................................................................................................... 3300/4932
[00:47:49] .................................................................................................... 3400/4932
[00:47:52] ....i............................................................................................... 3500/4932
[00:47:53] .......................i............................................................................ 3600/4932
[00:47:55] ........F...................................................................i..FF................... 3700/4932
[00:47:59] .................................................................................................... 3900/4932
[00:48:02] .................................................................................................... 4000/4932
[00:48:05] .................................................................................................... 4100/4932
[00:48:09] ...................................i................................................................ 4200/4932
---
[00:48:27] .................................................................................................... 4700/4932
[00:48:30] .................................................................................................... 4800/4932
48:33] ---- [ui] ui/parser/mod_file_not_exist.rs stdout ----
[00:48:33] normalized stderr:
[00:48:33] error[E0583]: file not found for module `not_a_real_file`
[00:48:33]    |
[00:48:33]    |
[00:48:33] LL | mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`
[00:48:33]    |
[00:48:33]    |
[00:48:33]    = help: name the file either not_a_real_file.rs or not_a_real_file/mod.rs inside the directory "$DIR"
[00:48:33] error: aborting due to previous error
[00:48:33] 
[00:48:33] For more information about this error, try `rustc --explain E0583`.
[00:48:33] 
[00:48:33] 
[00:48:33] 
[00:48:33] 
[00:48:33] The actual stderr differed from the expected stderr.
[00:48:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist/mod_file_not_exist.stderr
[00:48:33] To update references, rerun the tests and pass the `--bless` flag
[00:48:33] To only update this specific test, also pass `--test-args parser/mod_file_not_exist.rs`
[00:48:33] error: 1 errors occurred comparing output.
[00:48:33] status: exit code: 1
[00:48:33] status: exit code: 1
[00:48:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mod_file_not_exist.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/builst.rs:15:5\n   |\nLL | mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`\n   |     ^^^^^^^^^^^^^^^\n   |\n   = help: name the file either not_a_real_file.rs or not_a_real_file/mod.rs inside the directory \"/checkout/src/test/ui/parser\"\n\n"}
[00:48:33] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:33] {"message":"For more information about this error, try `rustc --explain E0583`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0583`.\n"}
[00:48:33] ------------------------------------------
[00:48:33] 
[00:48:33] thread '[ui] ui/parser/mod_file_not_exist.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:48:33] 
[00:48:33] 
[00:48:33] ---- [ui] ui/parser/mod_file_with_path_attr.rs stdout ----
[00:48:33] diff of stderr:
[00:48:33] 
[00:48:33] - error: couldn't read "C:/msys64/home/we/rust/src/test/ui/parser/not_a_real_file.rs": The system cannot find the file specified. (os error 2)
[00:48:33] + error: couldn't read "$DIR/not_a_real_file.rs": No such file or directory (os error 2)
[00:48:33] 3    |
[00:48:33] 3    |
[00:48:33] 4 LL | mod m; //~ ERROR not_a_real_file.rs
[00:48:33] 
[00:48:33] The actual stderr differed from the expected stderr.
[00:48:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_with_path_attr/mod_file_with_path_attr.stderr
[00:48:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_with_path_attr/mod_file_with_path_attr.stderr
[00:48:33] To update references, rerun the tests and pass the `--bless` flag
[00:48:33] To only update this specific test, also pass `--test-args parser/mod_file_with_path_attr.rs`
[00:48:33] error: 1 errors occurred comparing output.
[00:48:33] status: exit code: 1
[00:48:33] status: exit code: 1
[00:48:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mod_file_with_path_attr.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_with_path_attr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_with_path_attr/auxiliary" "-A" "unused"
[00:48:33] ------------------------------------------
[00:48:33] 
[00:48:33] ------------------------------------------
[00:48:33] stderr:
[00:48:33] stderr:
[00:48:33] ------------------------------------------
[00:48:33] {"message":"couldn't read \"/checkout/src/test/ui/parser/not_a_real_file.rs\": No such file or directory (os error 2)","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/mod_file_with_path_attr.rs","byte_start":535,"byte_end":536,"line_start":14,"line_end":14,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"mod m; //~ ERROR not_a_real_file.rs","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: couldn't read \"/checkout/src/test/ui/parser/not_a_real_file.rs\": No such file or directory (os error 2)\n  --> /checkout/src/test/ui/parser/mod_file_with_path_attr.rs:14:5\n   |\nLL | mod m; //~ ERROR not_a_real_file.rs\n   |     ^\n\n"}
[00:48:33] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:33] ------------------------------------------
[00:48:33] 
[00:48:33] thread '[ui] ui/parser/mod_file_with_path_attr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:48:33] 
---
[00:48:33] 
[00:48:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:48:33] 
[00:48:33] 
[00:48:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:33] 
[00:48:33] 
[00:48:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:33] Build completed unsuccessfully in 0:03:36
[00:48:33] Build completed unsuccessfully in 0:03:36
[00:48:33] make: *** [check] Error 1
[00:48:33] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0201912a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1020dd8a:start=1540078613759074516,finish=1540078613764840969,duration=5766453
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01debde0
$ ln -s . checkout && for CORE 
