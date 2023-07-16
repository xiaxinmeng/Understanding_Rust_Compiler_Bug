\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/try-block/try-block-bad-type.rs","byte_start":479,"byte_end":484,"line_start":19,"line_end":19,"column_start":24,"column_end":29,"is_primary":true,"text":[{"text":"    let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: std::ops::Try` is not satisfied","highlight_start":24,"highlight_end":29}],"label":"the trait `std::ops::Try` is not implemented for `i32`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/try-block/try-block-bad-type.rs","byte_start":479,"byte_end":484,"line_start":19,"line_end":19,"column_start":24,"column_end":29,"is_primary":false,"text":[{"text":"    let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: std::ops::Try` is not satisfied","highlight_start":24,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `try block`","def_site_span":{"file_name":"/checkout/src/test/ui/try-block/try-block-bad-type.rs","byte_start":479,"byte_end":484,"line_start":19,"line_end":19,"column_start":24,"column_end":29,"is_primary":false,"text":[{"text":"    let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: std::ops::Try` is not satisfied","highlight_start":24,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"required by `std::ops::Try::from_ok`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `i32: std::ops::Try` is not satisfied\n  --> /checkout/src/test/ui/try-block/try-block-bad-type.rs:19:24\n   |\nLL |     let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: std::ops::Try` is not satisfied\n   |                        ^^^^^ the trait `std::ops::Try` is not implemented for `i32`\n   |\n   = note: required by `std::ops::Try::from_ok`\n\n"}
[01:06:20] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[01:06:20] {"message":"Some errors occurred: E0271, E0277.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0271, E0277.\n"}
[01:06:20] 
[01:06:20] ------------------------------------------
[01:06:20] 
[01:06:20] thread '[ui] ui/try-block/try-block-bad-type.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:06:20] 
[01:06:20] 
[01:06:20] 
[01:06:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:06:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-unknown-linux-gnueabihf" "--mode" "ui" "--target" "arm-unknown-linux-gnueabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-linux-gnueabihf-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:20] 
[01:06:20] 
[01:06:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
[01:06:20] Build completed unsuccessfully in 1:02:44
---
travis_time:end:264f3912:start=1547811150419271592,finish=1547811150430131735,duration=10860143
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22fd2d80
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06a2d996
travis_time:start:06a2d996
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00cb4630
$ dmesg | grep -i kill
