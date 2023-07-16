\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-object.rs","byte_start":602,"byte_end":619,"line_start":18,"line_end":18,"column_start":13,"column_end":30,"is_primary":true,"text":[{"text":"    let _: &dyn IteratorAlias = &vec![123].into_iter();","highlight_start":13,"highlight_end":30}],"label":"associated type `Item` must be specified","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/rustc/42741ddb5182fac9cb0afb1f8425ae963b0b9db4/src/libcore/iter/iterator.rs","byte_start":4605,"byte_end":4615,"line_start":104,"line_end":104,"column_start":5,"column_end":15,"is_primary":false,"text":[{"text":"","highlight_start":5,"highlight_end":15}],"label":"`Item` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0191]: the value of the associated type `Item` (from the trait `std::iter::Iterator`) must be specified\n  --> /checkout/src/test/ui/traits/trait-alias-object.rs:18:13\n   |\nLL |     let _: &dyn IteratorAlias = &vec![123].into_iter();\n   |             ^^^^^^^^^^^^^^^^^ associated type `Item` must be specified\n\n"}
[00:48:35] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:35] {"message":"Some errors occurred: E0038, E0191.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0038, E0191.\n"}
[00:48:35] {"message":"For more information about an error, try `rustc --explain E0038`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0038`.\n"}
[00:48:35] ------------------------------------------
[00:48:35] 
[00:48:35] thread '[ui] ui/traits/trait-alias-object.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:35] 
---
[00:48:35] test result: FAILED. 4991 passed; 5 failed; 27 ignored; 0 measured; 0 filtered out
[00:48:35] 
[00:48:35] 
[00:48:35] 
[00:48:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-x86_64/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:35] 
[00:48:35] 
[00:48:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[00:48:35] Build completed unsuccessfully in 0:45:35
---
travis_time:end:0f6952b6:start=1542511722366367750,finish=1542511722373289323,duration=6921573
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b1a5af8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b74614c
travis_time:start:0b74614c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f5b4d48
$ dmesg | grep -i kill
