\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/type/type-dependent-def-issue-49241.rs","byte_start":54,"byte_end":55,"line_start":3,"line_end":3,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    const l: usize = v.count(); //~ ERROR can't capture dynamic environment in a fn item","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the `|| { ... }` closure form instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0434]: can't capture dynamic environment in a fn item\n  --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:3:22\n   |\nLL |     const l: usize = v.count(); //~ ERROR can't capture dynamic environment in a fn item\n   |                      ^\n   |\n   = help: use the `|| { ... }` closure form instead\n\n"}
[01:00:58] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/type/type-dependent-def-issue-49241.rs","byte_start":139,"byte_end":140,"line_start":4,"line_end":4,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"    let s: [u32; l] = v.into_iter().collect();","highlight_start":18,"highlight_end":19}],"label":"referenced constant has errors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"chil/const-len-underflow-separate-spans.rs
[01:00:58]     [ui] ui/consts/const-tup-index-span.rs
[01:00:58]     [ui] ui/error-codes/E0080.rs
[01:00:58]     [ui] ui/eval-enum.rs
[01:00:58]     [ui] ui/infinite/infinite-recursion-const-fn.rs
---
[01:00:58] test result: FAILED. 5320 passed; 19 failed; 23 ignored; 0 measured; 0 filtered out
[01:00:58] 
[01:00:58] 
[01:00:58] 
[01:00:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:58] 
[01:00:58] 
[01:00:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:58] Build completed unsuccessfully in 0:04:21
[01:00:58] Build completed unsuccessfully in 0:04:21
[01:00:58] make: *** [check] Error 1
[01:00:58] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:001b65e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 15:27:19 UTC 2019
---
travis_time:end:378a11c4:start=1549207641200001366,finish=1549207641207446288,duration=7444922
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14d87b4c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:171663b7
$ dmesg | grep -i kill
