plain
[00:49:56] ---- [ui] ui/issues/issue-57198.rs stdout ----
[00:49:56] diff of stderr:
[00:49:56] 
[00:49:56] 3    |
[00:49:56] 4 LL |     m::for();
[00:49:56] 5    |        ^^^ expected identifier, found keyword
[00:49:56] + help: you can escape reserved keywords to use them as identifiers
[00:49:56] +    |
[00:49:56] + LL |     m::r#for();
[00:49:56] 6 
[00:49:56] 7 error: aborting due to previous error
[00:49:56] 8 
[00:49:56] 
[00:49:56] 
[00:49:56] 
[00:49:56] The actual stderr differed from the expected stderr.
[00:49:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57198/issue-57198.stderr
[00:49:56] To update references, rerun the tests and pass the `--bless` flag
[00:49:56] To only update this specific test, also pass `--test-args issues/issue-57198.rs`
[00:49:56] error: 1 errors occurred comparing output.
[00:49:56] status: exit code: 1
[00:49:56] status: exit code: 1
[00:49:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-57198.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57198/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57198/auxiliary" "-A" "unused"
[00:49:56] ------------------------------------------
[00:49:56] 
[00:49:56] ------------------------------------------
[00:49:56] stderr:
[00:49:56] stderr:
[00:49:56] ------------------------------------------
[00:49:56] {"message":"expected identifier, found keyword `for`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57198.rs","byte_start":52,"byte_end":55,"line_start":6,"line_end":6,"column_start":8,"column_end":11,"is_primary":true,"text":[{"text":"    m::for();","highlight_start":8,"highlight_end":11}],"label":"expected identifier, found keyword","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can escape reserved keywords to use them as identifiers","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57198.rs","byte_start":52,"byte_end":55,"line_start":6,"line_end":6,"column_start":8,"column_end":11,"is_primary":true,"text":[{"text":"    m::for();","highlight_start":8,"highlight_end":11}],"label":null,"suggested_replacement":"r#for","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: expected identifier, found keyword `for`\n  --> /checkout/src/test/ui/issues/issue-57198.rs:6:8\n   |\nLL |     m::for();\n   |        ^^^ expected identifier, found keyword\nhelp: you can escape reserved keywords to use them as identifiers\n   |\nLL |     m::r#for();\n   |        ^^^^^\n\n"}
[00:49:56] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:56] ------------------------------------------
[00:49:56] 
[00:49:56] thread '[ui] ui/issues/issue-57198.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:49:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:49:56] 
[00:49:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[00:49:56] 
[00:49:56] 
[00:49:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-x86_64/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:56] 
[00:49:56] 
[00:49:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[00:49:56] Build completed unsuccessfully in 0:48:03
---
travis_time:end:1faa4569:start=1546266586385388138,finish=1546266586395825473,duration=10437335
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0581c344
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0da90481
travis_time:start:0da90481
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b147694
$ dmesg | grep -i kill
