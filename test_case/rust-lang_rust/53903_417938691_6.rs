\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/union-ub-fat-ptr.rs","byte_start":3697,"byte_end":3780,"line_start":125,"line_end":125,"column_start":1,"column_end":84,"is_primary":true,"text":[{"text":"const I2: &MySliceBool = &MySlice(unsafe { BoolTransmute { val: 3 }.bl }, [false]);","highlight_start":1,"highlight_end":84}],"label":"type validation failed: encountered 3 at .<deref>.0, but expected something in the range 0..=1","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: this constant likely exhibits undefined behavior\n  --> /checkout/src/test/ui/union-ub-fat-ptr.rs:125:1\n   |\nLL | const I2: &MySliceBool = &MySlice(unsafe { BoolTransmute { val: 3 }.bl }, [false]);\n   |  believe it should not be considered undefined behavior\n\n"}
[00:48:01] {"message":"aborting due to 14 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 14 previous errors\n\n"}
[00:48:01] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[00:48:01] ------------------------------------------
[00:48:01] 
[00:48:01] thread '[ui] ui/union-ub-fat-ptr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:48:01] 
---
[00:48:01] 
[00:48:01] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:48:01] 
[00:48:01] 
[00:48:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-l CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:012ad08e
travis_time:start:012ad08e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2aa9d8aa
$ dmesg | grep -i kill
