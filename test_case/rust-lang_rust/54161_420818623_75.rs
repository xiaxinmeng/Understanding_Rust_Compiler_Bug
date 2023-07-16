\n\nThis syntax specifies that we want the X type from MyTrait, as made concrete in\nMyStruct. The reason that we cannot simply use `MyStruct::X` is that MyStruct\nmight implement two different traits with identically-named associated types.\nThis syntax allows disambiguation between the two.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-item-privacy.rs","byte_start":3591,"byte_end":3595,"line_start":129,"line_end":129,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"    let _: S::C; //~ ERROR ambiguous associated type","highlight_start":12,"highlight_end":16}],"label":"ambiguous associated type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"specify the type using the syntax `<S as Trait>::C`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0223]: ambiguous associated type\n  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:129:12\n   |\nLL |     let _: S::C; //~ ERROR ambiguous associated type\n   |            ^^^^ ambiguous associated type\n   |\n   = note: specify the type using the syntax `<S as Trait>::C`\n\n"}
[00:51:58] {"message":"associated type `A` is private","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-item-privacy.rs","byte_start":3730,"byte_end":3734,"line_start":131,"line_end":131,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"    let _: T::A; //~ ERROR associated type `A` is private","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: associated type `A` is private\n  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:131:12\n   |\nLL |     let _: T::A; //~ ERROR associated type `A` is private\n   |            ^^^^\n\n"}
[00:51:58] {"message":"associated type `A` is private","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-item-privacy.rs","byte_start":3933,"byte_end":3939,"line_start":140,"line_end":140,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        A = u8, //~ ERROR associated type `A` is private","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: associated type `A` is private\n  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:140:9\n   |\nLL |         A = u8, //~ ERROR associated type `A` is private\n   |         ^^^^^^\n\n"}
[00:51:58] {"message":"aborting due to 17 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 17 previous errors\n\n"}
[00:51:58] {"message":"Some errors occurred: E0038, E0223, E0277, E0599, E0624.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0038, E0223, E0277, E0599, E0624.\n"}
[00:51:58] {"message":"For more information about an error, try `rustc --explain E0038`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0038`.\n"}
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] thread '[ui] ui/traits/trait-item-privacy.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:51:58] 
---
[00:51:58] test result: FAILED. 6742 passed; 4 failed; 28 ignored; 0 measured; 0 filtered out
[00:51:58] 
[00:51:58] 
[00:51:58] 
[00:51:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:58] 
[00:51:58] 
[00:51:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:58] Build completed unsuccessfully in 0:07:30
[00:51:58] Build completed unsuccessfully in 0:07:30
[00:51:58] make: *** [check] Error 1
[00:51:58] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0025a898
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
