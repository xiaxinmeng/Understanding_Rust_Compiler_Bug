\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/normalization.rs","byte_start":260,"byte_end":262,"line_start":12,"line_end":12,"column_start":31,"column_end":33,"is_primary":true,"text":[{"text":"    let b: <() as Foo>::Out = &a; //~ ERROR","highlight_start":31,"highlight_end":33}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/normalization.rs","byte_start":274,"byte_end":275,"line_start":13,"line_end":13,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`a` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/normalization.rs","byte_start":241,"byte_end":257,"line_start":12,"line_end":12,"column_start":12,"column_end":28,"is_primary":false,"text":[{"text":"    let b: <() as Foo>::Out = &a; //~ ERROR","highlight_start":12,"highlight_end":28}],"label":"type annotation requires that `a` is borrowed for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0597]: `a` does not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/normalization.rs:12:31\n   |\nLL |     let b: <() as Foo>::Out = &a; //~ ERROR\n   |            ----------------   ^^ borrowed value does not live long enough\n   |            |\n   |            type annotation requires that `a` is borrowed for `'static`\nLL | }\n   | - `a` dropped here while still borrowed\n\n"}
[00:47:05] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:05] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:47:05] ------------------------------------------
[00:47:05] 
[00:47:05] thread '[ui] ui/nll/user-annotations/normalization.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:47:05] 
---
[00:47:05] test result: FAILED. 4631 passed; 3 failed; 24 ignored; 0 measured; 0 filtered out
[00:47:05] 
[00:47:05] 
[00:47:05] 
[00:47:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:05] 
[00:47:05] 
[00:47:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:47:05] Build completed unsuccessfully in 0:43:53
---
travis_time:end:00264960:start=1540047072912473454,finish=1540047072919205154,duration=6731700
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:180ad742
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0456ac05
travis_time:start:0456ac05
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e7f430c
$ dmesg | grep -i kill
