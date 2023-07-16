\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":831,"byte_end":835,"line_start":27,"line_end":27,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    send(cycle2().clone());","highlight_start":5,"highlight_end":9}],"label":"`std::rc::Rc<std::string::String>` cannot be sent between threads safely","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"within `impl std::clone::Clone`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::string::String>`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"required because it appears within the type `impl std::clone::Clone`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required by `send`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":533,"byte_end":555,"line_start":16,"line_end":16,"column_start":1,"column_end":23,"is_primary":true,"text":[{"text":"fn send<T: Send>(_: T) {}","highlight_start":1,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `std::rc::Rc<std::string::String>: std::marker::Send` is tered out
[00:50:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:14] 
[00:50:14] 
[00:50:14] 
[00:50:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:14] 
[00:50:14] 
[00:50:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:14] Build completed unsuccessfully in 0:02:14
[00:50:14] Build completed unsuccessfully in 0:02:14
[00:50:14] make: *** [check] Error 1
[00:50:14] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00398752
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
