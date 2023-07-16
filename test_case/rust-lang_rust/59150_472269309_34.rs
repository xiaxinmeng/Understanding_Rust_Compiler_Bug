\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs","byte_start":223,"byte_end":225,"line_start":14,"line_end":14,"column_start":7,"column_end":9,"is_primary":true,"text":[{"text":"    }.hi() { //~ ERROR expected one of `.`, `;`, `?`, `}`, or an operator, found `{`","highlight_start":7,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0599]: no method named `hi` found for type `()` in the current scope\n  --> /checkout/src/test/ui/parser/struct-literal-restrictions-in-lamda.rs:14:7\n   |\nLL |     }.hi() { //~ ERROR expected one of `.`, `;`, `?`, `}`, or an operator, found `{`\n   |       ^^\n\n"}
[01:17:14] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:17:14] {"message":"Some errors occurred: E0423, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0423, E0599.\n"}
[01:17:14] 
[01:17:14] ------------------------------------------
[01:17:14] 
[01:17:14] thread '[ui] ui/parser/struct-literal-restrictions-in-lamda.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:14] thread '[ui] ui/parser/struct-literal-restrictions-in-lamda.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:14] 
[01:17:14] ---- [ui] ui/type/type-ascription-instead-of-statement-end.rs stdout ----
[01:17:14] diff of stderr:
[01:17:14] 
[01:17:14] 18    |
[01:17:14] 19 LL |     println!("test"): 0;
[01:17:14] 20    |     ^^^^^^^^^^^^^^^^
[01:17:14] - note: ...due to this, which is why a type is expected
[01:17:14] + note: ...due to this, which is why a type is expected after
[01:17:14] 23    |
[01:17:14] 23    |
[01:17:14] 24 LL |     println!("test"): 0;
[01:17:14] 
[01:17:14] The actual stderr differed from the expected stderr.
[01:17:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-instead-of-statement-end/type-ascription-instead-of-statement-end.stderr
[01:17:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-instead-of-statement-end/type-ascription-instead-of-statement-end.stderr
[01:17:14] To update references, rerun the tests and pass the `--bless` flag
[01:17:14] To only update this specific test, also pass `--test-args type/type-ascription-instead-of-statement-end.rs`
[01:17:14] error: 1 errors occurred comparing output.
[01:17:14] status: exit code: 1
[01:17:14] status: exit code: 1
[01:17:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-ascription-instead-of-statement-end.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-instead-of-statement-end/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-ascription-instead-of-statement-end/auxiliary" "-A" "unused"
[01:17:14] ------------------------------------------
[01:17:14] 
[01:17:14] ------------------------------------------
[01:17:14] stderr:
[01:17:14] stderr:
[01:17:14] ------------------------------------------
[01:17:14] {"message":"expected type, found `0`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/type/type-ascription-instead-of-statement-end.rs","byte_start":68,"byte_end":69,"line_start":5,"line_end":5,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    0; //~ ERROR expected type, found `0`","highlight_start":5,"highlight_end":6}],"label":"expecting a type here because of type ascription","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a semicolon","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/type/type-ascription-instead-of-statement-end.rs","byte_start":62,"byte_end":63,"line_start":4,"line_end":4,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"    println!(\"test\"):","highlight_start":21,"highlight_end":22}],"label":null,"suggested_replacement":";","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: expected type, found `0`\n  --> /checkout/src/test/ui/type/type-ascription-instead-of-statement-end.rs:5:5\n   |\nLL |     println!(\"test\"):\n   |                     - help: try using a semicolon: `;`\nLL |     0; //~ ERROR expected type, found `0`\n   |     ^ expecting a type here because of type ascription\n\n"}
[01:17:14] {"message":"expected type, found `0`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/type/type-ascription-instead-of-statement-end.rs","byte_start":142,"byte_end":143,"line_start":9,"line_end":9,"column_start":23,"column_end":24,"is_primary":true,"text":[{"text":"    println!(\"test\"): 0; //~ ERROR expected type, found `0`","highlight_start":23,"highlight_end":24}],"label":"expecting a type here because of type ascription","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"type ascription is a nightly only feature that lets you annotate expressions with a type: `<expr>: <type>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this expression is annotated with type ascription...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/type/type-ascription-instead-of-statement-end.rs","byte_start":124,"byte_end":140,"line_start":9,"line_end":9,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    println!(\"test\"): 0; //~ ERROR expected type, found `0`","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...due to this, which is why a type is expected after","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/type/type-ascription-instead-of-statement-end.rs","byte_start":140,"byte_end":141,"line_start":9,"line_end":9,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"    println!(\"test\"): 0; //~ ERROR expected type, found `0`","highlight_start":21,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"this might be indicative of a syntax error elsewhere","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: expected type, found `0`\n  --> /checkout/src/test/ui/type/type-ascription-instead-of-statement-end.rs:9:23\n   |\nLL |     println!(\"test\"): 0; //~ ERROR expected type, found `0`\n   |                       ^ expecting a type here because of type ascription\n   |\n   = note: type ascription is a nightly only feature that lets you annotate expressions with a type: `<expr>: <type>`\nnote: this expression is annotated with type ascription...\n  --> /checkout/src/test/ui/type/type-ascription-instead-of-statement-end.rs:9:5\n   |\nLL |     println!(\"test\"): 0; //~ ERROR expected type, found `0`\n   |     ^^^^^^^^^^^^^^^^\nnote: ...due to this, which is why a type is expected after\n  --> /checkout/src/test/ui/type/type-ascription-instead-of-statement-end.rs:9:21\n   |\nLL |     println!(\"test\"): 0; //~ ERROR expected type, found `0`\n   |                     ^\n   = help: this might be indicative of a syntax error elsewhere\n\n"}
[01:17:14] 
[01:17:14] ------------------------------------------
[01:17:14] 
[01:17:14] thread '[ui] ui/type/type-ascription-instead-of-statement-end.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:17:14] 
[01:17:14] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:17:14] 
[01:17:14] 
[01:17:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:14] 
[01:17:14] 
[01:17:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:14] Build completed unsuccessfully in 0:04:51
[01:17:14] Build completed unsuccessfully in 0:04:51
[01:17:14] make: *** [check] Error 1
[01:17:14] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03020c08
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 13 03:50:55 UTC 2019
---
travis_time:end:30a8eb1d:start=1552449057854729926,finish=1552449057859701971,duration=4972045
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11f5da00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bde469d
travis_time:start:0bde469d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:016839c3
$ dmesg | grep -i kill
