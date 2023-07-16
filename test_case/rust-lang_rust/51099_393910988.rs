plain
[00:44:05] ....................................................................................................
[00:44:10] ...............................................................................................i....
[00:44:15] ..........................................................................i.........................
[00:44:20] ....................................................................................................
[00:44:25] ....................................F...............................................................
[00:44:30] ....................................................................................................
[00:44:34] ......i.................iiiiiiiii...................................................
[00:44:34] 
[00:44:34] ---- [ui] ui/similar-tokens.rs stdout ----
[00:44:34] diff of stderr:
[00:44:34] 
[00:44:34] 
[00:44:34] - error: expected one of `,`, `::`, or `as`, found `.`
[00:44:34] + error: expected one of `,`, `::`, `as`, or `}`, found `.`
[00:44:34] 3    |
[00:44:34] 3    |
[00:44:34] - LL | use x::{A. B}; //~ ERROR expected one of `,`, `::`, `as`, or `}`, found `.`
[00:44:34] + LL | use x::{A. B}; //~ ERROR expected one of `,`, `::`, or `as`, found `.`
[00:44:34] 5    |          ^ expected one of `,`, `::`, `as`, or `}` here
[00:44:34] 7 error: aborting due to previous error
[00:44:34] 
[00:44:34] 
[00:44:34] The actual stderr differed from the expected stderr.
[00:44:34] The actual stderr differed from the expected stderr.
[00:44:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/similar-tokens/similar-tokens.stderr
[00:44:34] To update references, rerun the tests and pass the `--bless` flag
[00:44:34] To only update this specific test, also pass `--test-args similar-tokens.rs`
[00:44:34] error: 1 errors occurred comparing output.
[00:44:34] status: exit code: 101
[00:44:34] status: exit code: 101
[00:44:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/similar-tokens.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/similar-tokens/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/similar-tokens/auxiliary" "-A" "unused"
[00:44:34] ------------------------------------------
[00:44:34] 
[00:44:34] ------------------------------------------
[00:44:34] stderr:
[00:44:34] stderr:
[00:44:34] ------------------------------------------
[00:44:34] {"message":"expected one of `,`, `::`, `as`, or `}`, found `.`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/similar-tokens.rs","byte_start":595,"byte_end":596,"line_start":17,"line_end":17,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"use x::{A. B}; //~ ERROR expected one of `,`, `::`, or `as`, found `.`","highlight_start":10,"highlight_end":11}],"label":"expected one of `,`, `::`, `as`, or `}` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `,`, `::`, `as`, or `}`, found `.`\n  --> /checkout/src/test/ui/similar-tokens.rs:17:10\n   |\nLL | use x::{A. B}; //~ ERROR expected one of `,`, `::`, or `as`, found `.`\n   |          ^ expected one of `,`, `::`, `as`, or `}` here\n\n"}
[00:44:34] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:34] ------------------------------------------
[00:44:34] 
[00:44:34] thread '[ui] ui/similar-tokens.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:44:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:44:34] 
[00:44:34] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:44:34] 
[00:44:34] 
[00:44:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:34] 
[00:44:34] 
[00:44:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:34] Build completed unsuccessfully in 0:02:30
[00:44:34] Build completed unsuccessfully in 0:02:30
[00:44:34] make: *** [check] Error 1
[00:44:34] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b83b9a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
