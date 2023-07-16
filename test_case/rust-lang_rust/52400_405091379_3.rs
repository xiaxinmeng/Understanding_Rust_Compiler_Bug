\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/match-test-ptr-null.rs","byte_start":856,"byte_end":857,"line_start":17,"line_end":17,"column_start":13,"column_end":14,"is_primary":false,"text":[{"text":"            0 => 42, //~ ERROR constant contains unimplemented expression type","highlight_start":13,"highlight_end":14}],"label":"\"pointer arithmetic or comparison\" needs an rfc before being allowed inside constants","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/const-eval/match-test-ptr-null.rs","byte_start":707,"byte_end":1049,"line_start":15,"line_end":21,"column_start":26,"column_end":6,"is_primary":true,"text":[{"text":"    let _: [u8; 0] = [4; { //~ ERROR could not evaluate repeat length","highlight_start":26,"highlight_end":70},{"text":"        match &1 as *const i32 as usize { //~ ERROR raw pointers cannot be cast to integers","highlight_start":1,"highlight_end":92},{"text":"            0 => 42, //~ ERROR constant contains unimplemented expression type","highlight_start":1,"highlight_end":79},{"text":"            //~^ NOTE \"pointer arithmetic or comparison\" needs an rfc before being allowed","highlight_start":1,"highlight_end":91},{"text":"            n => n,","highlight_start":1,"highlight_end":20},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }];","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: could not evaluate repeat length\n  --> /checkout/src/test/ui/const-eval/match-test-ptr-null.rs:15:26\n   |\nLL |       let _: [u8; 0] = [4; { //~ ERROR could not evaluate repeat length\n   |  __________________________^\nLL | |         match &1 as *const i32 as usize { //~ ERROR raw pointers cannot be cast to integers\nLL | |             0 => 42, //~ ERROR constant contains unimplemented expression type\n   | |             - \"pointer arithmetic or comparison\" needs an rfc before being allowed inside constants\nLL | |             //~^ NOTE \"pointer arithmetic or comparison\" needs an rfc before being allowed\nLL | |             n => n,\nLL | |         }\nLL | |     }];\n   | |_____^\n\n"}
[00:46:39] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:46:39] {"message":"Some errors occurred: E0018, E0019, E0080.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0018, E0019, E0080.\n"}
[00:46:39] {"message":"For more information about an error, try `rustc --explain E0018`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0018`.\n"}
[00:46:39] ------------------------------------------
[00:46:39] 
[00:46:39] 
[00:46:39] thread '[ui] ui/const-eval/match-test-ptr-null.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:46:39] 
[00:46:39] 
[00:46:39] failures:
[00:46:39] failures:
[00:46:39]     [ui] ui/const-eval/match-test-ptr-null.rs
[00:46:39] test result: FAILED. 1562 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[00:46:39] 
[00:46:39] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:46:39] 
[00:46:39] 
[00:46:39] 
[00:46:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:39] 
[00:46:39] 
[00:46:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:39] Build completed unsuccessfully in 0:01:27
[00:46:39] Build completed unsuccessfully in 0:01:27
[00:46:39] Makefile:58: recipe for target 'check' failed
[00:46:39] make: *** [check] Error 1
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0175cb35
travis_time:start:0175cb35
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03e59fea
travis_time:start:03e59fea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b61b9f1
$ dmesg | grep -i kill
