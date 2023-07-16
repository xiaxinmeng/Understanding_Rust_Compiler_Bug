\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32963.rs","byte_start":114,"byte_end":139,"line_start":8,"line_end":8,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    size_of_copy::<Misc+Copy>();","highlight_start":5,"highlight_end":30}],"label":"the trait `std::marker::Copy` is not implemented for `dyn Misc`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `size_of_copy`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32963.rs","byte_start":30,"byte_end":72,"line_start":5,"line_end":5,"column_start":1,"column_end":43,"is_primary":true,"text":[{"text":"fn size_of_copy<T: Copy+?Sized>() -> usize { mem::size_of::<T>() }","highlight_start":1,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `dyn Misc: std::marker::Copy` is not satisfied\n  --> /checkout/src/test/ui/issues/issue-32963.rs:8:5\n   |\nLL |     size_of_copy::<Misc+Copy>();\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `dyn Misc`\n   |\nnote: required by `size_of_copy`\n  --> /checkout/src/test/ui/issues/issue-32963.rs:5:1\n   |\nLL | fn size_of_copy<T: Copy+?Sized>() -> usize { mem::size_of::<T>() }\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:15:35] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:15:35] {"message":"Some errors occurred: E0225, E0277.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0225, E0277.\n"}
[01:15:35] 
[01:15:35] ------------------------------------------
[01:15:35] 
[01:15:35] thread '[ui] ui/issues/issue-32963.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:15:35] thread '[ui] ui/issues/issue-32963.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:15:35] 
[01:15:35] ---- [ui] ui/traits/trait-alias/trait-alias-no-sized.rs stdout ----
[01:15:35] 
[01:15:35] error: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs:12: unexpected error: '12:12: 12:18: at least one non-builtin trait is required for an object type [E0224]'
[01:15:35] 
[01:15:35] error: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs:26: unexpected error: '26:12: 26:18: at least one non-builtin trait is required for an object type [E0224]'
[01:15:35] 
[01:15:35] error: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs:12: expected error not found: `?Trait` is not permitted in trait object types
[01:15:35] 
[01:15:35] error: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs:16: expected error not found: `?Trait` is not permitted in trait object types
[01:15:35] 
[01:15:35] error: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs:20: expected error not found: `?Trait` is not permitted in trait object types
[01:15:35] 
[01:15:35] error: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs:26: expected error not found: `?Trait` is not permitted in trait object types
[01:15:35] 
[01:15:35] error: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs:26: expected error not found: `?Trait` is not permitted in trait object types
[01:15:35] 
[01:15:35] error: /checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs:26: expected error not found: only auto traits can be used as additional traits in a trait object [E0225]
[01:15:35] error: 2 unexpected errors found, 6 expected errors not found
[01:15:35] status: exit code: 1
[01:15:35] status: exit code: 1
[01:15:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias/trait-alias-no-sized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-no-sized/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-no-sized/auxiliary" "-A" "unused"
[01:15:35]     Error {
[01:15:35]         line_num: 12,
[01:15:35]         kind: Some(
[01:15:35]             Error
---
[01:15:35]         line_num: 12,
[01:15:35]         kind: Some(
[01:15:35]             Error
[01:15:35]         ),
[01:15:35]         msg: "`?Trait` is not permitted in trait object types"
[01:15:35]     Error {
[01:15:35]         line_num: 16,
[01:15:35]         kind: Some(
[01:15:35]             Error
[01:15:35]             Error
[01:15:35]         ),
[01:15:35]         msg: "`?Trait` is not permitted in trait object types"
[01:15:35]     Error {
[01:15:35]         line_num: 20,
[01:15:35]         kind: Some(
[01:15:35]             Error
[01:15:35]             Error
[01:15:35]         ),
[01:15:35]         msg: "`?Trait` is not permitted in trait object types"
[01:15:35]     Error {
[01:15:35]         line_num: 26,
[01:15:35]         kind: Some(
[01:15:35]             Error
[01:15:35]             Error
[01:15:35]         ),
[01:15:35]         msg: "`?Trait` is not permitted in trait object types"
[01:15:35]     Error {
[01:15:35]         line_num: 26,
[01:15:35]         kind: Some(
[01:15:35]             Error
[01:15:35]             Error
[01:15:35]         ),
[01:15:35]         msg: "`?Trait` is not permitted in trait object types"
[01:15:35]     Error {
[01:15:35]         line_num: 26,
[01:15:35]         kind: Some(
[01:15:35]             Error
[01:15:35]             Error
[01:15:35]         ),
[01:15:35]         msg: "only auto traits can be used as additional traits in a trait object [E0225]"
[01:15:35] ]
[01:15:35] 
[01:15:35] 
[01:15:35] thread '[ui] ui/traits/trait-alias/trait-alias-no-sized.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1381:13
[01:15:35] 
[01:15:35] failures:
[01:15:35]     [ui] ui/associated-types/associated-types-overridden-binding-2.rs
[01:15:35]     [ui] ui/bad/bad-sized.rs
[01:15:35]     [ui] ui/bad/bad-sized.rs
[01:15:35]     [ui] ui/error-codes/E0225.rs
[01:15:35]     [ui] ui/issues/issue-22560.rs
[01:15:35]     [ui] ui/issues/issue-32963.rs
[01:15:35]     [ui] ui/traits/trait-alias/trait-alias-no-sized.rs
[01:15:35] test result: FAILED. 5505 passed; 6 failed; 21 ignored; 0 measured; 0 filtered out
[01:15:35] 
[01:15:35] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:15:35] 
[01:15:35] 
[01:15:35] 
[01:15:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:35] 
[01:15:35] 
[01:15:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:35] Build completed unsuccessfully in 0:04:53
[01:15:35] Build completed unsuccessfully in 0:04:53
[01:15:35] make: *** [check] Error 1
[01:15:35] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0eacfc6f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr  3 18:02:30 UTC 2019
---
travis_time:end:1aed7e97:start=1554314552912349130,finish=1554314552918148012,duration=5798882
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00c71820
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20fc13a0
travis_time:start:20fc13a0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0079c09c
$ dmesg | grep -i kill
