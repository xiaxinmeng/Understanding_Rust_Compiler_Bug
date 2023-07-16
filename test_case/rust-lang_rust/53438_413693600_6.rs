\n\nNote: This error is usually hidden by E0301 or E0302. This may change in the\nfuture.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-2.rs","byte_start":1410,"byte_end":1412,"line_start":38,"line_end":38,"column_start":18,"column_end":20,"is_primary":true,"text":[{"text":"                (|| { *x = None; drop(force_fn_once); })();","highlight_start":18,"highlight_end":20}],"label":"cannot mutably borrow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-2.rs","byte_start":1250,"byte_end":1251,"line_start":33,"line_end":33,"column_start":11,"column_end":12,"is_primary":false,"text":[{"text":"    match x {","highlight_start":11,"highlight_end":12}],"label":"value is immutable in match guard","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-2.rs","byte_start":1416,"byte_end":1417,"line_start":38,"line_end":38,"column_start":24,"column_end":25,"is_primary":false,"text":[{"text":"                (|| { *x = None; drop(force_fn_once); })();","highlight_start":24,"highlight_end":25}],"label":"borrow occurs due to use of `x` in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0510]: cannot mutably borrow `x` in match guard\n  --> /checkout/src/test/ui/issues/issue-27282-mutate-before-diverging-arm-2.rs:38:18\n   |\nLL |     match x {\n   |           - value is immutable in match guard\n...\nLL |                 (|| { *x = None; drop(force_fn_once); })();\n   |                  ^^    - borrow occurs duec-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:18] 
[00:48:18] 
[00:48:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:18] Build completed unsuccessfully in 0:03:12
[00:48:18] Build completed unsuccessfully in 0:03:12
[00:48:18] make: *** [check] Error 1
[00:48:18] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04b70b24
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:083e754a:start=1534455711505465015,finish=1534455711513945812,duration=8480797
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:20f3cc41
$ ln -s . checkout && f
