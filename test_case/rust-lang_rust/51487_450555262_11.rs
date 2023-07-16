\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-declaration-type.rs","byte_start":1050,"byte_end":1100,"line_start":37,"line_end":37,"column_start":9,"column_end":59,"is_primary":true,"text":[{"text":"        fn x86_mm_adds_epu16(x: i16x8, y: i16x8) -> i16x8;","highlight_start":9,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0442]: intrinsic return value has wrong type: found `i16`, expected `u16`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-declaration-type.rs:37:9\n   |\nLL |         fn x86_mm_adds_epu16(x: i16x8, y: i16x8) -> i16x8;\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:01:44] {"message":"aborting due to 12 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 12 previous errors\n\n"}
[01:01:44] {"message":"For more information about this error, try `rustc --explain E0442`.","code":null,"level":"","spans":[],"children":[],"rendered":"For moreux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:44] 
[01:01:44] 
[01:01:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:44] Build completed unsuccessfully in 0:03:55
[01:01:44] Build completed unsuccessfully in 0:03:55
[01:01:44] make: *** [check] Error 1
[01:01:44] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b912c70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec 30 11:45:26 UTC 
