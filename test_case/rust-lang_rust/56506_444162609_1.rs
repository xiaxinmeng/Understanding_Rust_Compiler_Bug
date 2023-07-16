\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/edition-imports-2018.rs","byte_start":83,"byte_end":117,"line_start":6,"line_end":6,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"extern crate edition_imports_2015;","highlight_start":1,"highlight_end":35}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `edition_imports_2015`\n  --> /checkout/src/test/ui/proc-macro/edition-imports-2018.rs:6:1\n   |\nLL | extern crate edition_imports_2015;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
[00:45:42] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:42] {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
[00:45:42] ------------------------------------------
[00:45:42] 
[00:45:42] thread '[ui] ui/proc-macro/edition-imports-2018.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:45:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:42] 
[00:45:42] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:45:42] 
[00:45:42] 
[00:45:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:42] 
[00:45:42] 
[00:45:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:45:42] Build completed unsuccessfully in 0:43:34
---
travis_time:end:00dc611c:start=1543940667519128148,finish=1543940667525837707,duration=6709559
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10dcb4fe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a065210
travis_time:start:0a065210
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:076be390
$ dmesg | grep -i kill
