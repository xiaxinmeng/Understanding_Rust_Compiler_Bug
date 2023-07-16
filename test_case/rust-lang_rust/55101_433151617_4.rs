\n\nIt is also possible to overload most operators for your own type by\nimplementing traits from `std::ops`.\n\nString concatenation appends the string on the right to the string on the\nleft and may require reallocation. This requires ownership of the string\non the left. If something should be added to a string literal, move the\nliteral to the heap by allocating it with `to_owned()` like in\n`\"Your text\".to_owned()`.\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/traits/trait-alias-bounds.rs","byte_start":1640,"byte_end":1652,"line_start":56,"line_end":56,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    *x == 22_i32","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`impl SendEqAlias<i32>` might need a bound for `std::cmp::PartialEq`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0369]: binary operation `==` cannot be applied to type `impl SendEqAlias<i32>`\n  --> /checkout/src/test/run-pass/traits/trait-alias-bounds.rs:56:5\n   |\nLL |     *x == 22_i32\n   |     ^^^^^^^^^^^^\n   |\n   = note: `impl SendEqAlias<i32>` might need a bound for `std::cmp::PartialEq`\n\n"}
[00:57:41] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:57:41] {"message":"For more information about this error, try `rustc --explain E0369`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0369`.\n"}
[00:57:41] ------------------------------------------
[00:57:41] 
[00:57:41] thread '[run-pass] run-pass/traits/trait-alias-bounds.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:57:41] 
---
[00:57:41] 
[00:57:41] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:57:41] 
[00:57:41] 
[00:57:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib"122,duration=45639077
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:011259fe
---
travis_time:end:067b29f0:start=1540491009984116936,finish=1540491010058175149,duration=74058213
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1cbbef6e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:022b665b
$ dmesg | grep -i kill
