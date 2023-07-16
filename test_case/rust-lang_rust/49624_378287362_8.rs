\n\nwhere `P1, ..., Pm` are the type parameters of the `impl` and `T0, ..., Tn`\nare types. One of the types `T0, ..., Tn` must be a local type (this is another\norphan rule, see the explanation for E0117). Let `i` be the smallest integer\nsuch that `Ti` is a local type. Then no type parameter can appear in any of the\n`Tj` for `j < i`.\n\nFor information on the design of the orphan rules, see [RFC 1023].\n\n[RFC 1023]: https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/e0119/issue-28981.rs","byte_start":502,"byte_end":525,"line_start":15,"line_end":15,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"impl<Foo> Deref for Foo { } //~ ERROR must be used","highlight_start":1,"highlight_end":24}],"label":"type parameter `Foo` must be used as the type parameter for some local type","suggested_replacement":null,"expansion":null}],"children":[{"message":"only traits defined in the current crate can be implemented for a type parameter","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0210]: type parameter `Foo` must be used as the type parameter for some local type (e.g. `MyStruct<Foo>`)\n  --> /checkout/src/test/ui/e0119/issue-28981.rs:15:1\n   |\nLL | impl<Foo> Deref for Foo { } //~ ERROR must be used\n   | ^^^^^^^^^^^^^^^^^^^^^^^ type parameter `Foo` must be used as the type parameter for some local type\n   |\n   = note: only traits defined in the current crate can be implemented for a type parameter\n\n"}
[00:41:40] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:41:40] {"message":"Some errors occurred: E0119, E0210.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0119, E0210.\n"}
[00:41:40] {"message":"For more information about an error, try `rustc --explain E0119`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0119`.\n"}
---
[00:41:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:41:40] expected success, got: exit code: 101
[00:41:40]
[00:41:40]
[00:41:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:41:40] Build completed unsuccessfully in 0:01:55
[00:41:40] make: *** [check] Error 1
[00:41:40] Makefile:58: recipe for target 'check' failed
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:03fbf6a0:start=1522768647643915371,finish=1522768647651434788,duration=7519417
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02de74ef
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:02de74ef:start=1522768647658523738,finish=1522768647665064969,duration=6541231
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13ba59ab
$ dmesg | grep -i kill
[   10.885261] init: failsafe main process (1094) killed by TERM signal
