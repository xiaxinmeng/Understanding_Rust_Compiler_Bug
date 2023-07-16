plain
[01:07:44] failures:
[01:07:44] 
[01:07:44] ---- [run-pass] run-pass/asm-in-moved.rs#ast stdout ----
[01:07:44] normalized stderr:
[01:07:44] warning: struct is never constructed: `NoisyDrop`
[01:07:44]    |
[01:07:44]    |
[01:07:44] LL | struct NoisyDrop<'a>(&'a Cell<&'static str>);
[01:07:44]    |
[01:07:44]    = note: #[warn(dead_code)] on by default
[01:07:44] 
[01:07:44] 
[01:07:44] 
[01:07:44] 
[01:07:44] 
[01:07:44] The actual stderr differed from the expected stderr.
[01:07:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/asm-in-moved.ast/asm-in-moved.ast.stderr
[01:07:44] To update references, rerun the tests and pass the `--bless` flag
[01:07:44] To only update this specific test, also pass `--test-args asm-in-moved.rs`
[01:07:44] 
[01:07:44] error in revision `ast`: 1 errors occurred comparing output.
[01:07:44] status: exit code: 0
[01:07:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/asm-in-moved.rs" "--target=wasm32-unknown-unknown" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/asm-in-moved.ast/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/asm-in-moved.ast/auxiliary"
[01:07:44] ------------------------------------------
[01:07:44] 
[01:07:44] ------------------------------------------
[01:07:44] stderr:
[01:07:44] stderr:
[01:07:44] ------------------------------------------
[01:07:44] {"message":"struct is never constructed: `NoisyDrop`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/asm-in-moved.rs","byte_start":584,"byte_end":629,"line_start":19,"line_end":19,"column_start":1,"column_end":46,"is_primary":true,"text":[{"text":"struct NoisyDrop<'a>(&'a Cell<&'static str>);","highlight_start":1,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `NoisyDrop`\n  --> /checkout/src/test/run-pass/asm-in-moved.rs:19:1\n   |\nLL | struct NoisyDrop<'a>(&'a Cell<&'static str>);\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:07:44] ------------------------------------------
[01:07:44] 
[01:07:44] 
[01:07:44] thread '[run-pass] run-pass/asm-in-moved.rs#ast' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:07:44] 
[01:07:44] ---- [run-pass] run-pass/asm-in-moved.rs#mir stdout ----
[01:07:44] normalized stderr:
[01:07:44] normalized stderr:
[01:07:44] warning: struct is never constructed: `NoisyDrop`
[01:07:44]    |
[01:07:44]    |
[01:07:44] LL | struct NoisyDrop<'a>(&'a Cell<&'static str>);
[01:07:44]    |
[01:07:44]    = note: #[warn(dead_code)] on by default
[01:07:44] 
[01:07:44] 
[01:07:44] 
[01:07:44] 
[01:07:44] 
[01:07:44] The actual stderr differed from the expected stderr.
[01:07:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/asm-in-moved.mir/asm-in-moved.mir.stderr
[01:07:44] To update references, rerun the tests and pass the `--bless` flag
[01:07:44] To only update this specific test, also pass `--test-args asm-in-moved.rs`
[01:07:44] 
[01:07:44] error in revision `mir`: 1 errors occurred comparing output.
[01:07:44] status: exit code: 0
[01:07:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/asm-in-moved.rs" "--target=wasm32-unknown-unknown" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/asm-in-moved.mir/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/asm-in-moved.mir/auxiliary"
[01:07:44] ------------------------------------------
[01:07:44] 
[01:07:44] ------------------------------------------
[01:07:44] stderr:
[01:07:44] stderr:
[01:07:44] ------------------------------------------
[01:07:44] {"message":"struct is never constructed: `NoisyDrop`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/asm-in-moved.rs","byte_start":584,"byte_end":629,"line_start":19,"line_end":19,"column_start":1,"column_end":46,"is_primary":true,"text":[{"text":"struct NoisyDrop<'a>(&'a Cell<&'static str>);","highlight_start":1,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `NoisyDrop`\n  --> /checkout/src/test/run-pass/asm-in-moved.rs:19:1\n   |\nLL | struct NoisyDrop<'a>(&'a Cell<&'static str>);\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:07:44] ------------------------------------------
[01:07:44] 
[01:07:44] 
[01:07:44] thread '[run-pass] run-pass/asm-in-moved.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:07:44] ---- [run-pass] run-pass/simple_global_asm.rs stdout ----
[01:07:44] normalized stderr:
[01:07:44] normalized stderr:
[01:07:44] warning: foreign function is never used: `foo`
[01:07:44]    |
[01:07:44]    |
[01:07:44] LL |     fn foo();
[01:07:44]    |
[01:07:44]    = note: #[warn(dead_code)] on by default
[01:07:44] 
[01:07:44] 
[01:07:44] 
[01:07:44] 
[01:07:44] 
[01:07:44] The actual stderr differed from the expected stderr.
[01:07:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simple_global_asm/simple_global_asm.stderr
[01:07:44] To update references, rerun the tests and pass the `--bless` flag
[01:07:44] To only update this specific test, also pass `--test-args simple_global_asm.rs`
[01:07:44] error: 1 errors occurred comparing output.
[01:07:44] status: exit code: 0
[01:07:44] status: exit code: 0
[01:07:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/simple_global_asm.rs" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simple_global_asm/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simple_global_asm/auxiliary"
[01:07:44] ------------------------------------------
[01:07:44] 
[01:07:44] ------------------------------------------
[01:07:44] stderr:
[01:07:44] stderr:
[01:07:44] ------------------------------------------
[01:07:44] {"message":"foreign function is never used: `foo`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/simple_global_asm.rs","byte_start":665,"byte_end":674,"line_start":24,"line_end":24,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    fn foo();","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: foreign function is never used: `foo`\n  --> /checkout/src/test/run-pass/simple_global_asm.rs:24:5\n   |\nLL |     fn foo();\n   |     ^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:07:44] ------------------------------------------
[01:07:44] 
[01:07:44] thread '[run-pass] run-pass/simple_global_asm.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:07:44] 
---
[01:07:44] test result: FAILED. 478 passed; 3 failed; 68 ignored; 0 measured; 0 filtered out
[01:07:44] 
[01:07:44] 
[01:07:44] 
[01:07:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-pass" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:44] 
[01:07:44] 
[01:07:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:07:44] Build completed unsuccessfully in 1:03:34
---
travis_time:end:16c5cb56:start=1537362540201594213,finish=1537362540207243852,duration=5649639
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:002d0848
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15b84d66
travis_time:start:15b84d66
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1772bc6f
$ dmesg | grep -i kill
