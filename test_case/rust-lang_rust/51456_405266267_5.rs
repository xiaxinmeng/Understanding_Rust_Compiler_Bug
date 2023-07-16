\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/crate-in-paths.rs","byte_start":586,"byte_end":589,"line_start":19,"line_end":19,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    Foo;","highlight_start":5,"highlight_end":8}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"possible candidate is found in another module, you can import it into scope","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/crate-in-paths.rs","byte_start":535,"byte_end":535,"line_start":14,"line_end":14,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod bar {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use bar::Foo;\n\n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0425]: cannot find value `Foo` in this scope\n  --> /checkout/src/test/ui/crate-in-paths.rs:19:5\n   |\nLL |     Foo;\n   |     ^^^ not found in this scope\nhelp: possible candidate is found in a.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:42:11] 
[00:42:11] 
[00:42:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:42:11] Build completed unsuccessfully in 0:01:25
[00:42:11] Build completed unsuccessfully in 0:01:25
[00:42:11] Makefile:58: recipe for target 'check' failed
[00:42:11] make: *** [check] Error 1
52376 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
52372 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
52028 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
49440 ./src/test
---
travis_time:end:14a68e97:start=1531751319561442800,finish=1531751319568679438,duration=7236638
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:31accfe7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:022c2b9e
travis_time:start:022c2b9e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12b7c08c
$ dmesg | grep -i kill
