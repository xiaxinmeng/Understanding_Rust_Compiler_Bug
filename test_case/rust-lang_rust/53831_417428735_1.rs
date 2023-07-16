\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/ub-ptr-in-usize.rs","byte_start":582,"byte_end":636,"line_start":19,"line_end":19,"column_start":1,"column_end":55,"is_primary":true,"text":[{"text":"const PTR_AS_USIZE: usize = unsafe { Foo { a: &1 }.b};","highlight_start":1,"highlight_end":55}],"label":"type validation failed: encountered a pointer, but expected the type usize","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: this constant likely exhibits undefined behavior\n  --> /checkout/src/test/ui/consts/const-eval/ub-ptr-in-usize.rs:19:1\n   |\nLL | const PTR_AS_USIZE: usize = unsafe { Foo { a: &1 }.b};\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected the type usize\n   |\n   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior\n\n"}
[00:52:28] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:52:28] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[00:52:28] ------------------------------------------
[00:52:28] 
[00:52:28] 
[00:52:28] thags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:28] 
[00:52:28] 
[00:52:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:28] Build completed unsuccessfully in 0:03:21
[00:52:28] Build completed unsuccessfully in 0:03:21
[00:52:28] make: *** [check] Error 1
[00:52:28] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03ff7587
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0beabae2:start=1535655199657479966,finish=1535655199667837244,duration=10357278
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:103c6542
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ffb9f64
$ dmesg | grep -i kill
