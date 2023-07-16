\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-use-in-index-lvalue.rs","byte_start":719,"byte_end":723,"line_start":20,"line_end":20,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    w[5] = 0; //[ast]~ ERROR use of possibly uninitialized variable: `*w` [E0381]","highlight_start":5,"highlight_end":9}],"label":"use of possibly uninitialized `*w`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0381]: use of possibly uninitialized variable: `*w`\n  --> /checkout/src/test/ui/borrowck/borrowck-use-in-index-lvalue.rs:20:5\n   |\nLL |     w[5] = 0; //[ast]~ ERROR use of possibly uninitialized variable: `*w` [E0381]\n   |     ^^^^ use of possibly uninitialized `*w`\n\n"}
[00:51:50] {"message":"cannot assign to `w[..]` when `w` is not initialized","code":{"code":"E0718","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-use-in-index-lvalue.rs","byte_start":719,"byte_end":727,"line_start":20,"line_end":20,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    w[5] = 0; //[ast]~ ERROR use of possibly uninitialized variable: `*w` [E0381]","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0718]: cannot assign to `w[..]` when `w` is not initialized\n  --> /checkout/src/test/ui/borrowck/borrowck-use-in-index-lvalue.rs:20:5\n   |\nLL |     w[5] = 0; //[ast]~ ERROR use of possibly uninitialized variable: `*w` [E0381]\n   |     ^^^^^^^^\n\n"}
[00:51:50] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:51:50] {"message":"Some errors occurred: E0381, E0718.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0381, E0718.\n"}
[00:51:50] {"message":"For more information about an error, try `rustc --explain E0381`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0381`.\n"}
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] thread '[ui] ui/borrowck/borrowck-use-in-index-lvalue.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:51:50] 
---
[00:51:50] test result: FAILED. 4556 passed; 3 failed; 20 ignored; 0 measured; 0 filtered out
[00:51:50] 
[00:51:50] 
[00:51:50] 
[00:51:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/p4327288 .
2479164 ./obj/build
1846652 ./obj/build/x86_64-unknown-linux-gnu
1069648 ./src
777924 ./.git
---
151464 ./obj/build/bootstrap/debug/incremental
151412 ./src/tools/clang
149124 ./src/llvm-emscripten/test
135996 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113
135992 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113/s-f5klit5ye0-9w4ug0-1akhp35ixzhyj
135632 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
135628 ./objE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09e5c6e9
travis_time:start:09e5c6e9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16c1bf3c
$ dmesg | grep -i kill
