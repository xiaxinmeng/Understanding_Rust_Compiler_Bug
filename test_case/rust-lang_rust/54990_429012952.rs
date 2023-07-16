plain
[00:45:23] 
[00:45:23] ---- [ui] ui/conditional-compilation/cfg-attr-multi-true.rs stdout ----
[00:45:23] diff of stderr:
[00:45:23] 
[00:45:23] 24 LL |         MustUseDeprecated {} //~ warning: use of deprecated item
[00:45:23] 26 
[00:45:23] 26 
[00:45:23] - warning: unused `MustUseDeprecated` which must be used
[00:45:23] + warning: unused `MustUseDeprecated` that must be used
[00:45:23] 29    |
[00:45:23] 29    |
[00:45:23] 30 LL |     MustUseDeprecated::new(); //~ warning: use of deprecated item
[00:45:23] 
[00:45:23] The actual stderr differed from the expected stderr.
[00:45:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/cfg-attr-multi-true.stderr
[00:45:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/cfg-attr-multi-true.stderr
[00:45:23] To update references, rerun the tests and pass the `--bless` flag
[00:45:23] To only update this specific test, also pass `--test-args conditional-compilation/cfg-attr-multi-true.rs`
[00:45:23] error: 1 errors occurred comparing output.
[00:45:23] status: exit code: 0
[00:45:23] status: exit code: 0
[00:45:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/auxiliary" "-A" "unused"
[00:45:23] ------------------------------------------
[00:45:23] 
[00:45:23] ------------------------------------------
[00:45:23] stderr:
[00:45:23] stderr:
[00:45:23] ------------------------------------------
[00:45:23] {"message":"use of deprecated item 'MustUseDeprecated'","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":334,"byte_end":351,"line_start":13,"line_end":13,"column_start":6,"column_end":23,"is_primary":true,"text":[{"text":"impl MustUseDeprecated { //~ warning: use of deprecated item","highlight_start":6,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(deprecated)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:13:6\n   |\nLL | impl MustUseDeprecated { //~ warning: use of deprecated item\n   |      ^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(deprecated)] on by default\n\n"}
[00:45:23] {"message":"use of deprecated item 'MustUseDeprecated'","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":552,"byte_end":574,"line_start":20,"line_end":20,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    MustUseDeprecated::new(); //~ warning: use of deprecated item","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:20:5\n   |\nLL |     MustUseDeprecated::new(); //~ warning: use of deprecated item\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:45:23] {"message":"use of deprecated item 'MustUseDeprecated'","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":406,"byte_end":423,"line_start":14,"line_end":14,"column_start":17,"column_end":34,"is_primary":true,"text":[{"text":"    fn new() -> MustUseDeprecated { //~ warning: use of deprecated item","highlight_start":17,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:14:17\n   |\nLL |     fn new() -> MustUseDeprecated { //~ warning: use of deprecated item\n   |                 ^^^^^^^^^^^^^^^^^\n\n"}
[00:45:23] {"message":"use of deprecated item 'MustUseDeprecated'","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":470,"byte_end":487,"line_start":15,"line_end":15,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"        MustUseDeprecated {} //~ warning: use of deprecated item","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:15:9\n   |\nLL |         MustUseDeprecated {} //~ warning: use of deprecated item\n   |         ^^^^^^^^^^^^^^^^^\n\n"}
[00:45:23] {"message":"unused `MustUseDeprecated` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":552,"byte_end":577,"line_start":20,"line_end":20,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    MustUseDeprecated::new(); //~ warning: use of deprecated item","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":212,"byte_end":227,"line_start":7,"line_end":7,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![warn(unused_must_use)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused `MustUseDeprecated` that must be used\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:20:5\n   |\nLL |     MustUseDeprecated::new(); //~ warning: use of deprecated item\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:7:9\n   |\nLL | #![warn(unused_must_use)]\n   |         ^^^^^^^^^^^^^^^\n\n"}
[00:45:23] ------------------------------------------
[00:45:23] 
[00:45:23] thread '[ui] ui/conditional-compilation/cfg-attr-multi-true.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:45:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:23] 
[00:45:23] ---- [ui] ui/lint/must_use-unit.rs stdout ----
[00:45:23] diff of stderr:
[00:45:23] 
[00:45:23] - error: unused return value of `foo` which must be used
[00:45:23] + error: unused return value of `foo` that must be used
[00:45:23] 3    |
[00:45:23] 3    |
[00:45:23] 4 LL |     foo(); //~ unused return value of `foo`
[00:45:23] 
[00:45:23] 10 LL | #![deny(unused_must_use)]
[00:45:23] 12 
[00:45:23] 12 
[00:45:23] - error: unused return value of `bar` which must be used
[00:45:23] + error: unused return value of `bar` that must be used
[00:45:23] 15    |
[00:45:23] 15    |
[00:45:23] 16 LL |     bar(); //~ unused return value of `bar`
[00:45:23] 
[00:45:23] The actual stderr differed from the expected stderr.
[00:45:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/must_use-unit.stderr
[00:45:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/must_use-unit.stderr
[00:45:23] To update references, rerun the tests and pass the `--bless` flag
[00:45:23] To only update this specific test, also pass `--test-args lint/must_use-unit.rs`
[00:45:23] error: 1 errors occurred comparing output.
[00:45:23] status: exit code: 1
[00:45:23] status: exit code: 1
[00:45:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_use-unit.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/auxiliary" "-A" "unused"
[00:45:23] ------------------------------------------
[00:45:23] 
[00:45:23] ------------------------------------------
[00:45:23] stderr:
[00:45:23] stderr:
[00:45:23] ------------------------------------------
[00:45:23] {"message":"unused return value of `foo` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":145,"byte_end":151,"line_start":14,"line_end":14,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    foo(); //~ unused return value of `foo`","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":33,"byte_end":48,"line_start":3,"line_end":3,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![deny(unused_must_use)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused return value of `foo` that must be used\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:14:5\n   |\nLL |     foo(); //~ unused return value of `foo`\n   |     ^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:3:9\n   |\nLL | #![deny(unused_must_use)]\n   |         ^^^^^^^^^^^^^^^\n\n"}
[00:45:23] {"message":"unused return value of `bar` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":190,"byte_end":196,"line_start":16,"line_end":16,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    bar(); //~ unused return value of `bar`","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unused return value of `bar` that must be used\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:16:5\n   |\nLL |     bar(); //~ unused return value of `bar`\n   |     ^^^^^^\n\n"}
[00:45:23] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:45:23] ------------------------------------------
[00:45:23] 
[00:45:23] thread '[ui] ui/lint/must_use-unit.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:45:23] 
---
[00:45:23] test result: FAILED. 4567 passed; 2 failed; 23 ignored; 0 measured; 0 filtered out
[00:45:23] 
[00:45:23] 
[00:45:23] 
[00:45:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-x86_64/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:23] 
[00:45:23] 
[00:45:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[00:45:23] Build completed unsuccessfully in 0:42:25
---
travis_time:end:240fca00:start=1539273540965673459,finish=1539273540971640363,duration=5966904
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:149c9685
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c93e9d8
travis_time:start:1c93e9d8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10069d3a
$ dmesg | grep -i kill
