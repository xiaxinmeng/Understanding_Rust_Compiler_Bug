plain
[00:59:44] 
[00:59:44] 1 error: cannot use a built-in attribute through an import
[00:59:44] 2   --> $DIR/cross-crate.rs:7:3
[00:59:44] 3    |
[00:59:44] - LL | #[built_in_attr] //~ ERROR cannot use a built-in attribute through an import
[00:59:44] + LL | #[built_in_attr]
[00:59:44] 6    |
[00:59:44] 7 note: the built-in attribute imported here
[00:59:44] 
[00:59:44] 13 error: cannot use a tool module through an import
[00:59:44] 13 error: cannot use a tool module through an import
[00:59:44] 14   --> $DIR/cross-crate.rs:8:3
[00:59:44] 15    |
[00:59:44] - LL | #[tool_mod::skip] //~ ERROR cannot use a tool module through an import
[00:59:44] + LL | #[tool_mod::skip]
[00:59:44] 18    |
[00:59:44] 19 note: the tool module imported here
[00:59:44] 
[00:59:44] 
[00:59:44] 
[00:59:44] The actual stderr differed from the expected stderr.
[00:59:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/cross-crate/cross-crate.stderr
[00:59:44] To update references, rerun the tests and pass the `--bless` flag
[00:59:44] To only update this specific test, also pass `--test-args rust-2018/uniform-paths/cross-crate.rs`
[00:59:44] error: 1 errors occurred comparing output.
[00:59:44] status: exit code: 1
[00:59:44] status: exit code: 1
[00:59:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/cross-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/cross-crate/auxiliary" "-A" "unused"
[00:59:44] ------------------------------------------
[00:59:44] 
[00:59:44] ------------------------------------------
[00:59:44] stderr:
[00:59:44] stderr:
[00:59:44] ------------------------------------------
[00:59:44] {"message":"cannot use a built-in attribute through an import","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs","byte_start":94,"byte_end":107,"line_start":7,"line_end":7,"column_start":3,"column_end":16,"is_primary":true,"text":[{"text":"#[built_in_attr] //~ ERROR cannot use a built-in attribute through an import","highlight_start":3,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the built-in attribute imported here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs","byte_start":75,"byte_end":89,"line_start":5,"line_end":5,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"use cross_crate::*;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot use a built-in attribute through an import\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs:7:3\n   |\nLL | #[built_in_attr] //~ ERROR cannot use a built-in attribute through an import\n   |   ^^^^^^^^^^^^^\n   |\nnote: the built-in attribute imported here\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs:5:5\n   |\nLL | use cross_crate::*;\n   |     ^^^^^^^^^^^^^^\n\n"}
[00:59:44] {"message":"cannot use a tool module through an import","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs","byte_start":171,"byte_end":179,"line_start":8,"line_end":8,"column_start":3,"column_end":11,"is_primary":true,"text":[{"text":"#[tool_mod::skip] //~ ERROR cannot use a tool module through an import","highlight_start":3,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the tool module imported here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs","byte_start":75,"byte_end":89,"line_start":5,"line_end":5,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"use cross_crate::*;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot use a tool module through an import\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs:8:3\n   |\nLL | #[tool_mod::skip] //~ ERROR cannot use a tool module through an import\n   |   ^^^^^^^^\n   |\nnote: the tool module imported here\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs:5:5\n   |\nLL | use cross_crate::*;\n   |     ^^^^^^^^^^^^^^\n\n"}
[00:59:44] 
[00:59:44] ------------------------------------------
[00:59:44] 
[00:59:44] thread '[ui] ui/rust-2018/uniform-paths/cross-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[00:59:44] 
[00:59:44] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[00:59:44] 
[00:59:44] 
[00:59:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:44] 
[00:59:44] 
[00:59:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:59:44] Build completed unsuccessfully in 0:56:33
---
travis_time:end:07d10be8:start=1552721571719834341,finish=1552721571730769526,duration=10935185
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02fd13e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:164883d0
travis_time:start:164883d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:026a71a7
$ dmesg | grep -i kill
