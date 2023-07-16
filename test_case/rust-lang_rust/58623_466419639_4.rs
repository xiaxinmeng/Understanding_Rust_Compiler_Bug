\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-21763.rs","byte_start":162,"byte_end":192,"line_start":9,"line_end":9,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    foo::<HashMap<Rc<()>, Rc<()>>>();","highlight_start":5,"highlight_end":35}],"label":"`std::rc::Rc<()>` cannot be sent between threads safely","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"within `(std::rc::Rc<()>, std::rc::Rc<()>)`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"required because it appears within the type `(std::rc::Rc<()>, std::rc::Rc<()>)`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(std::rc::Rc<()>, std::rc::Rc<()>)>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because it appears within the type `hashbrown::map::HashMap<std::rc::Rc<()>, std::rc::Rc<()>, std::collections::hash_map::RandomState>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because it appears within the type `std::collections::HashMap<std::rc::Rc<()>, std::rc::Rc<()>>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required by `foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-21763.rs","byte_start":124,"byte_end":141,"line_start":6,"line_end":6,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"fn foo<T: Send>() {}","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely\n  --> /checkout/src/test/ui/issues/issue-21763.rs:9:5\n   |\nLL |     foo::<HashMap<Rc<()>, Rc<()>>>();\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely\n   |\n   = help: within `(std::rc::Rc<()>, std::rc::Rc<()>)`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`\n   = note: required because it appears within the type `(std::rc::Rc<()>, std::rc::Rc<()>)`\n   = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(std::rc::Rc<()>, std::rc::Rc<()>)>`\n   = note: required because it appears within the type `hashbrown::map::HashMap<std::rc::Rc<()>, std::rc::Rc<()>, std::collections::hash_map::RandomState>`\n   = note: required because it appears within the type `std::collections::HashMap<std::rc::Rc<()>, std::rc::Rc<()>>`\nnote: required by `foo`\n  --> /checkout/src/test/ui/issues/issue-21763.rs:6:1\n   |\nLL | fn foo<T: Send>() {}\n   | ^^^^^^^^^^^^^^^^^\n\n"}
[01:15:08] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[01:15:08] 
[01:15:08] ------------------------------------------
[01:15:08] 
---
[01:15:08] 
[01:15:08] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:15:08] 
[01:15:08] 
[01:15:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:08] 
[01:15:08] 
[01:15:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:08] Build completed unsuccessfully in 0:04:14
[01:15:08] Build completed unsuccessfully in 0:04:14
[01:15:08] make: *** [check] Error 1
[01:15:08] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:044d37da
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 22 14:43:01 UTC 2019
---
travis_time:end:12c8ad82:start=1550846583040003467,finish=1550846583047892883,duration=7889416
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0706f548
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a593634
$ dmesg | grep -i kill
