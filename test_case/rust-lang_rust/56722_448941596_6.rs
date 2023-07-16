\n\n[PhantomData] can also be used to express information about unused type\nparameters.\n\n[PhantomData]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<T>>>>>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<T>>>>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<T>>>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required because of the requirements on the impl of `Foo` for `NoData<NoData<T>>`","code":null,"level":"note","spans":[],"children":[],"reData<NoData<T>>>>>>>>>>>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<T>>>>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<T>>>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<T>>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<NoData<T>>`\n   = note: required because of the requirements on the impl of `Foo` for `NoData<T>`\nnote: required by `Foo`\n  --> /checkout/src/test/ui/issues/issue-20413.rs:11:1\n   |\nLL | trait Foo {\n   | ^^^^^^^^^\n\n"}
[00:56:04] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:56:04] {"message":"Some errors occurred: E0275, E0392.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0275, E0392.\n"}
[00:56:04] {"message":"For more information about an error, try `rustc --explain E0275`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0275`.\n"}
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[ui] ui/issues/issue-20413.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:04] 
---
[00:56:04] test result: FAILED. 5162 passed; 4 failed; 23 ignored; 0 measured; 0 filtered out
[00:56:04] 
[00:56:04] 
[00:56:04] 
[00:56:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:04] 
[00:56:04] 
[00:56:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:04] Build completed unsuccessfully in 0:03:38
[00:56:04] Build completed unsuccessfully in 0:03:38
[00:56:04] Makefile:58: recipe for target 'check' failed
[00:56:04] make: *** [check] Error 1
2544612 ./obj
2544572 ./obj/build
1884656 ./obj/build/x86_64-unknown-linux-gnu
1119484 ./src
---
152196 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
152192 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
149800 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
144288 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144284 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7rfkouowc-17fd8za-3g9jcdhvsk79m
143776 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
124324 ./obj/bui:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18560972
travis_time:start:18560972
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f60d65c
$ dmesg | grep -i kill
