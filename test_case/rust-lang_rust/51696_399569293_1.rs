\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-37665.rs","byte_start":697,"byte_end":698,"line_start":20,"line_end":20,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"    let x: () = 0; //~ ERROR: mismatched types","highlight_start":17,"highlight_end":18}],"label":"expected (), found integral variable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `()`\n   found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/compile-fail/issue-37665.rs:20:17\n   |\nLL |     let x: () = 0; //~ ERROR: mismatched types\n   |                 ^ expected (), found integral variable\n   |\n   = note: expected type `()`\n              found type `{integer}`\n\n"}
[00:52:53] {"message":"librustc/middle/mem_categorization.rs:743: unexpected definition in memory categorization: Err","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-37665.rs","byte_start":630,"byte_end":633,"line_start":18,"line_end":18,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    foo.push(MAIN_SEPARATOR);","highlight_start":5,"highlight_end"::52:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:53] Build completed unsuccessfully in 0:13:10
[00:52:53] Makefile:58: recipe for target 'check' failed
[00:52:53] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:32b7d424
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:009bb5f0:start=1529698522613724782,finish=1529698522619727313,duration=6002531
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:019ced0e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d43f7ed
$ dmesg | grep -i kill
