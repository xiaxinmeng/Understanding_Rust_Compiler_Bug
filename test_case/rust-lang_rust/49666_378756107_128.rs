\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/get_default.rs","byte_start":1551,"byte_end":1554,"line_start":50,"line_end":50,"column_start":17,"column_end":20,"is_primary":true,"text":[{"text":"                map.set(String::new()); // Just AST errors here","highlight_start":17,"highlight_end":20}],"label":"mutable borrow occurs here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/get_default.rs","byte_start":1255,"byte_end":1258,"line_start":42,"line_end":42,"column_start":15,"column_end":18,"is_primary":false,"text":[{"text":"        match map.get() {","highlight_start":15,"highlight_end":18}],"label":"immutable borrow occurs here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/get_default.rs","byte_start":1684,"byte_end":1685,"line_start":55,"line_end":55,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"immutable borrow ends here","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable (Ast)\n  --> /checkout/src/test/ui/nll/get_default.rs:50:17\n   |\nLL |         match map.get() {\n   |               --- immutable borrow occurs here\n...\nLL |                 map.set(String::new()); // Just AST errors here\n   |                 ^^^ mutable borrow occurs here\n...\nLL | }\n   | - immutable borrow ends here\n\n"}
[00:42:43] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:42:43] {"message":"For more information about this error, try `rustc --explain E0502`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0502`.\n"}
---
[00:42:43] - error: internal compiler error: unexpected region for local data ReFree(DefId(0/0:3 ~ issue_48697[317d]::foo[0]), BrAnon(0))
[00:42:43] -   --> $DIR/issue-48697.rs:22:16
[00:42:43] -    |
[00:42:43] - LL |     let k = f(&z);
---
[00:42:43] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'nll/issue-48697.rs'
[00:42:43]
[00:42:43] error: 1 errors occurred comparing output.
[00:42:43] status: exit code: 0
[00:42:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-48697.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48697.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/check_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:42:43] expected success, got: exit code: 101
[00:42:43]
[00:42:43]
[00:42:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:42:43] Build completed unsuccessfully in 0:02:36
[00:42:43] Makefile:58: recipe for target 'check' failed
[00:42:43] make: *** [check] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0b810e38:start=1522878484570860216,finish=1522878484588336626,duration=17476410
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:077b3140
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:077b3140:start=1522878484598197237,finish=1522878484609219249,duration=11022012
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11a0a08f
$ dmesg | grep -i kill
[   10.518774] init: failsafe main process (1095) killed by TERM signal
