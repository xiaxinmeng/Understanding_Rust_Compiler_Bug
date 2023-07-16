\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/issue-52475.rs","byte_start":804,"byte_end":815,"line_start":20,"line_end":20,"column_start":17,"column_end":28,"is_primary":true,"text":[{"text":"            n = (n + 1) % 5;","highlight_start":17,"highlight_end":28}],"label":"duplicate interpreter state observed here, const evaluation will never terminate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: evaluation of constant value failed\n  --> /checkout/src/test/ui/consts/const-eval/issue-52475.rs:20:17\n   |\nLL |             n = (n + 1) % 5;\n   |                 ^^^^^^^^^^^ duplicate interpreter state observed here, const evaluation will never terminate\n\n"}
[00:46:41] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:41] {"message":"Some errors occurred: E0019, E0080.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0019, E0080.\n"}
[00:46:41] {"message":"For more information about an error, try `rustc --explain E0019`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0019`.\n"}
[00:46:41] ------------------------------------------
[00:46:41] 
[00:46:41] thread '[ui] ui/consts/const-eval/issue-52475.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:41] 
[00:46:41] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:46:41] 
[00:46:41] 
[00:46:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunsttravis_time:start:0a8c6270
Fri Sep  7 08:25:02 UTC 2018
Fri, 07 Sep 2018 08:25:02 GMT
travis_time:end:0a8c6270:start=1536308702797807691,finish=1536308702843685756,duration=45878065

---
travis_time:end:07fbaca4:start=1536308703913126239,finish=1536308703925905778,duration=12779539
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:061bafb6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14854fc4
$ dmesg | grep -i kill
