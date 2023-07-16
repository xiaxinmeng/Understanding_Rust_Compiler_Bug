\n\nIf the trait `Foo` was deriving from something like `Super<String>` or\n`Super<T>` (where `Foo` itself is `Foo<T>`), this is okay, because given a type\n`get_a()` will definitely return an object of that type.\n\nHowever, if it derives from `Super<Self>`, even though `Super` is object safe,\nthe method `get_a()` would return an object of unknown type when called on the\nfunction. `Self` type parameters let us make object safe traits no longer safe,\nso they are forbidden when specifying supertraits.\n\nThere's no easy fix for this, generally code will need to be refactored so that\nyou no longer need to derive from `Super<Self>`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/kindck/kindck-inherited-copy-bound.rs","byte_start":528,"byte_end":532,"line_start":28,"line_end":28,"column_start":19,"column_end":23,"is_primary":true,"text":[{"text":"    let z = &x as &Foo;","highlight_start":19,"highlight_end":23}],"label":"the trait `Foo` cannot be made into an object","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait cannot require that `Self : Sized`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required by cast to type '&dyn Foo'","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0038]: the trait `Foo` cannot be made into an object\n  --> /checkout/src/test/ui/kindck/kindck-inherited-copy-bound.rs:28:19\n   |\nLL |     let z = &x as &Foo;\n   |                   ^^^^ the trait `Foo` cannot be made into an object\n   |\n   = note: the trait cannot require that `Self : Sized`\n   = note: required by cast to type '&dyn Foo'\n\n"}
[01:16:54] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:16:54] {"message":"Some errors occurred: E0038, E0277.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0038, E0277.\n"}
[01:16:54] 
[01:16:54] ------------------------------------------
[01:16:54] 
[01:16:54] 
[01:16:54] thread '[ui] ui/kindck/kindck-inherited-copy-bound.rs#object_safe_for_dispatch' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:16:54] 
[01:16:54] failures:
[01:16:54]     [ui] ui/kindck/kindck-inherited-copy-bound.rs#curr
[01:16:54]     [ui] ui/kindck/kindck-inherited-copy-bound.rs#object_safe_for_dispatch
[01:16:54]     [ui] ui/kindck/kindck-inherited-copy-bound.rs#object_safe_for_dispatch
[01:16:54] 
[01:16:54] test result: FAILED. 5459 passed; 2 failed; 22 ignored; 0 measured; 0 filtered out
[01:16:54] 
[01:16:54] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:16:54] 
[01:16:54] 
[01:16:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:54] 
[01:16:54] 
[01:16:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:54] Build completed unsuccessfully in 0:04:53
[01:16:54] Build completed unsuccessfully in 0:04:53
[01:16:54] make: *** [check] Error 1
[01:16:54] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b79f2d9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 13 22:14:50 UTC 2019
---
travis_time:end:01b02a10:start=1552515292613324090,finish=1552515292618316240,duration=4992150
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:236f214c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:188b8840
travis_time:start:188b8840
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0441ae58
$ dmesg | grep -i kill
