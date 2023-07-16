\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generator/generator-region-requirements.rs","byte_start":364,"byte_end":365,"line_start":15,"line_end":15,"column_start":51,"column_end":52,"is_primary":true,"text":[{"text":"            GeneratorState::Complete(c) => return c,","highlight_start":51,"highlight_end":52}],"label":"lifetime `'static` required","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add explicit lifetime `'static` to the type of `x`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/generator/generator-region-requirements.rs","byte_start":180,"byte_end":188,"line_start":8,"line_end":8,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"fn dangle(x: &mut i32) -> &'static mut i32 {","highlight_start":14,"highlight_end":22}],"label":null,"suggested_replacement":"&'static mut i32","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0621]: explicit lifetime required in the type of `x`\n  --> /checkout/src/test/ui/generator/generator-region-requirements.rs:15:51\n   |\nLL | fn dangle(x: &mut i32) -> &'static mut i32 {\n   |              -------- help: add explicit lifetime `'static` to the type of `x`: `&'static mut i32`\n...\nLL |             GeneratorState::Complete(c) => return c,\n   |                                                   ^ lifetime `'static` required\n\n"}
[00:50:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:16] {"message":"For more information about this error, try `rustc --explain E0621`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0621`.\n"}
[00:50:16] ------------------------------------------
[00:50:16] 
[00:50:16] thread '[ui] ui/generator/generator-region-requirements.rs#nll' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:50:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:16] 
[00:50:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:50:16] 
[00:50:16] 
[00:50:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:16] 
[00:50:16] 
[00:50:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:50:16] Build completed unsuccessfully in 0:47:00
---
travis_time:end:13f77ce1:start=1540087140161480370,finish=1540087140170134568,duration=8654198
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23423140
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04911d65
travis_time:start:04911d65
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:073bbd94
$ dmesg | grep -i kill
