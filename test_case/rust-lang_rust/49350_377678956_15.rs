\n\nIf the trait `Foo` was deriving from something like `Super<String>` or\n`Super<T>` (where `Foo` itself is `Foo<T>`), this is okay, because given a type\n`get_a()` will definitely return an object of that type.\n\nHowever, if it derives from `Super<Self>`, even though `Super` is object safe,\nthe method `get_a()` would return an object of unknown type when called on the\nfunction. `Self` type parameters let us make object safe traits no longer safe,\nso they are forbidden when specifying supertraits.\n\nThere's no easy fix for this, generally code will need to be refactored so that\nyou no longer need to derive from `Super<Self>`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-40006.rs","byte_start":472,"byte_end":473,"line_start":11,"line_end":11,"column_start":6,"column_end":7,"is_primary":true,"text":[{"text":"impl X { //~ ERROR cannot be made into an object","highlight_start":6,"highlight_end":7}],"label":"the trait `X` cannot be made into an object","suggested_replacement":null,"expansion":null}],"children":[{"message":"method `xxx` has no receiver","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0038]: the trait `X` cannot be made into an object\n  --> /checkout/src/test/ui/did_you_mean/issue-40006.rs:11:6\n   |\nLL | impl X { //~ ERROR cannot be made into an object\n   |      ^ the trait `X` cannot be made into an object\n   |\n   = note: method `xxx` has no receiver\n\n"}
[00:43:17] {"message":"aborting due to 9 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 9 previous errors\n\n"}
[00:43:17] {"message":"For more information about this error, try `rustc --explain E0038`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0038`.\n"}
---
[00:43:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:17] expected success, got: exit code: 101
[00:43:17]
[00:43:17]
[00:43:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:17] Build completed unsuccessfully in 0:02:08
[00:43:17] make: *** [check] Error 1
[00:43:17] Makefile:58: recipe for target 'check' failed
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:18ce1ffa:start=1522487559984616300,finish=1522487559994137371,duration=9521071
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:217d89aa
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:217d89aa:start=1522487560007037380,finish=1522487560015300316,duration=8262936
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0373cbe1
$ dmesg | grep -i kill
[   10.895741] init: failsafe main process (1094) killed by TERM signal
