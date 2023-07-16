\n\nIf you don't know the basics of Rust, you can go look to the Rust Book to get\nstarted: https://doc.rust-lang.org/book/\n"},"level":"error","spans":[],"children":[{"message":"consider adding a `main` function to `/checkout/src/test/ui/directory_ownership/unowned_mod_with_path.rs`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0601]: `main` function not found in crate `unowned_mod_with_path`\n   |\n   = note: consider adding a `main` function to `/checkout/src/test/ui/directory_ownership/unowned_mod_with_path.rs`\n\n"}
[00:48:00] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:00] {"message":"For more information about this error, try `rustc --explain E0601`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0601`.\n"}
[00:48:00] ------------------------------------------
[00:48:00] 
[00:48:00] thread '[ui] ui/directory_ownership/unowned_mod_with_path.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:48:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:00] 
[00:48:00] ---- [ui] ui/directory_ownership/mod_file_not_owning.rs stdout ----
[00:48:00] 
[00:48:00] error: ui test compiled successfully!
[00:48:00] status: exit code: 0
[00:48:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/directory_ownership/mod_file_not_owning.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/directory_ownership/mod_file_not_owning/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/direct/runtest.rs:3196:9
[00:48:00] 
[00:48:00] failures:
[00:48:00]     [ui] ui/directory_ownership/mod_file_not_owning.rs
[00:48:00]     [ui] ui/directory_ownership/unowned_mod_with_path.rs
[00:48:00]     [ui] ui/directory_ownership/unowned_mod_with_path.rs
[00:48:00]     [ui] ui/non_modrs_mods/non_modrs_mods.rs
[00:48:00] 
[00:48:00] test result: FAILED. 4185 passed; 3 failed; 20 ignored; 0 measured; 0 filtered out
[00:48:00] 
[00:48:00] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:48:00] 
[00:48:00] 
[00:48:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2d592cce
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
