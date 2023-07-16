plain
[00:51:04] .................................................................................................... 300/4592
[00:51:07] .................................................................................................... 400/4592
[00:51:10] .................................................................................................... 500/4592
[00:51:14] ........i........................................................................................... 600/4592
[00:51:19] ....F............................................................................................... 700/4592
[00:51:28] ..........................................................iiiii..................................... 900/4592
[00:51:31] .................................................................................................... 1000/4592
[00:51:33] .................................................................................................... 1100/4592
[00:51:36] .................................................................................................... 1200/4592
---
[00:52:22] .................................................................................................... 2500/4592
[00:52:27] .................................................................................................... 2600/4592
[00:52:31] .................................................................................................... 2700/4592
[00:52:34] .................................................................................................... 2800/4592
[00:52:38] .............................................................................F...................... 2900/4592
[00:52:45] .................................................................................................... 3100/4592
[00:52:48] .......i.i..ii...................................................................................... 3200/4592
[00:52:53] .................................................................................................... 3300/4592
[00:52:56] ...........................................................i........................................ 3400/4592
---
[00:53:38] 
[00:53:38] ---- [ui] ui/conditional-compilation/cfg-attr-multi-true.rs stdout ----
[00:53:38] diff of stderr:
[00:53:38] 
[00:53:38] 24 LL |         MustUseDeprecated {} //~ warning: use of deprecated item
[00:53:38] 26 
[00:53:38] 26 
[00:53:38] - warning: unused `MustUseDeprecated` which must be used
[00:53:38] + warning: unused `MustUseDeprecated` that must be used
[00:53:38] 29    |
[00:53:38] 29    |
[00:53:38] 30 LL |     MustUseDeprecated::new(); //~ warning: use of deprecated item
[00:53:38] 
[00:53:38] The actual stderr differed from the expected stderr.
[00:53:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/cfg-attr-multi-true.stderr
[00:53:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/cfg-attr-multi-true.stderr
[00:53:38] To update references, rerun the tests and pass the `--bless` flag
[00:53:38] To only update this specific test, also pass `--test-args conditional-compilation/cfg-attr-multi-true.rs`
[00:53:38] error: 1 errors occurred comparing output.
[00:53:38] status: exit code: 0
[00:53:38] status: exit code: 0
[00:53:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg-attr-multi-true/auxiliary" "-A" "unused"
[00:53:38] ------------------------------------------
[00:53:38] 
[00:53:38] ------------------------------------------
[00:53:38] stderr:
[00:53:38] stderr:
[00:53:38] ------------------------------------------
[00:53:38] {"message":"use of deprecated item 'MustUseDeprecated'","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":334,"byte_end":351,"line_start":13,"line_end":13,"column_start":6,"column_end":23,"is_primary":true,"text":[{"text":"impl MustUseDeprecated { //~ warning: use of deprecated item","highlight_start":6,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(deprecated)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:13:6\n   |\nLL | impl MustUseDeprecated { //~ warning: use of deprecated item\n   |      ^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(deprecated)] on by default\n\n"}
[00:53:38] {"message":"use of deprecated item 'MustUseDeprecated'","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":552,"byte_end":574,"line_start":20,"line_end":20,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    MustUseDeprecated::new(); //~ warning: use of deprecated item","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:20:5\n   |\nLL |     MustUseDeprecated::new(); //~ warning: use of deprecated item\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:53:38] {"message":"use of deprecated item 'MustUseDeprecated'","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":406,"byte_end":423,"line_start":14,"line_end":14,"column_start":17,"column_end":34,"is_primary":true,"text":[{"text":"    fn new() -> MustUseDeprecated { //~ warning: use of deprecated item","highlight_start":17,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:14:17\n   |\nLL |     fn new() -> MustUseDeprecated { //~ warning: use of deprecated item\n   |                 ^^^^^^^^^^^^^^^^^\n\n"}
[00:53:38] {"message":"use of deprecated item 'MustUseDeprecated'","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":470,"byte_end":487,"line_start":15,"line_end":15,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"        MustUseDeprecated {} //~ warning: use of deprecated item","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:15:9\n   |\nLL |         MustUseDeprecated {} //~ warning: use of deprecated item\n   |         ^^^^^^^^^^^^^^^^^\n\n"}
[00:53:38] {"message":"unused `MustUseDeprecated` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":552,"byte_end":577,"line_start":20,"line_end":20,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    MustUseDeprecated::new(); //~ warning: use of deprecated item","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":212,"byte_end":227,"line_start":7,"line_end":7,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![warn(unused_must_use)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused `MustUseDeprecated` that must be used\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:20:5\n   |\nLL |     MustUseDeprecated::new(); //~ warning: use of deprecated item\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:7:9\n   |\nLL | #![warn(unused_must_use)]\n   |         ^^^^^^^^^^^^^^^\n\n"}
[00:53:38] ------------------------------------------
[00:53:38] 
[00:53:38] thread '[ui] ui/conditional-compilation/cfg-attr-multi-true.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:53:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:38] 
[00:53:38] ---- [ui] ui/lint/must_use-unit.rs stdout ----
[00:53:38] diff of stderr:
[00:53:38] 
[00:53:38] - error: unused return value of `foo` which must be used
[00:53:38] + error: unused return value of `foo` that must be used
[00:53:38] 3    |
[00:53:38] 3    |
[00:53:38] 4 LL |     foo(); //~ unused return value of `foo`
[00:53:38] 
[00:53:38] 10 LL | #![deny(unused_must_use)]
[00:53:38] 12 
[00:53:38] 12 
[00:53:38] - error: unused return value of `bar` which must be used
[00:53:38] + error: unused return value of `bar` that must be used
[00:53:38] 15    |
[00:53:38] 15    |
[00:53:38] 16 LL |     bar(); //~ unused return value of `bar`
[00:53:38] 
[00:53:38] The actual stderr differed from the expected stderr.
[00:53:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/must_use-unit.stderr
[00:53:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/must_use-unit.stderr
[00:53:38] To update references, rerun the tests and pass the `--bless` flag
[00:53:38] To only update this specific test, also pass `--test-args lint/must_use-unit.rs`
[00:53:38] error: 1 errors occurred comparing output.
[00:53:38] status: exit code: 1
[00:53:38] status: exit code: 1
[00:53:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_use-unit.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-unit/auxiliary" "-A" "unused"
[00:53:38] ------------------------------------------
[00:53:38] 
[00:53:38] ------------------------------------------
[00:53:38] stderr:
[00:53:38] stderr:
[00:53:38] ------------------------------------------
[00:53:38] {"message":"unused return value of `foo` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":145,"byte_end":151,"line_start":14,"line_end":14,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    foo(); //~ unused return value of `foo`","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":33,"byte_end":48,"line_start":3,"line_end":3,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![deny(unused_must_use)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused return value of `foo` that must be used\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:14:5\n   |\nLL |     foo(); //~ unused return value of `foo`\n   |     ^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:3:9\n   |\nLL | #![deny(unused_must_use)]\n   |         ^^^^^^^^^^^^^^^\n\n"}
[00:53:38] {"message":"unused return value of `bar` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":190,"byte_end":196,"line_start":16,"line_end":16,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    bar(); //~ unused return value of `bar`","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unused return value of `bar` that must be used\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:16:5\n   |\nLL |     bar(); //~ unused return value of `bar`\n   |     ^^^^^^\n\n"}
[00:53:38] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:53:38] ------------------------------------------
[00:53:38] 
[00:53:38] thread '[ui] ui/lint/must_use-unit.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:53:38] 
---
[00:53:38] 
[00:53:38] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:53:38] 
[00:53:38] 
[00:53:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:38] 
[00:53:38] 
[00:53:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:38] Build completed unsuccessfully in 0:03:52
[00:53:38] Build completed unsuccessfully in 0:03:52
[00:53:38] Makefile:58: recipe for target 'check' failed
[00:53:38] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03ef7a80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:267dad98:start=1539271774933487866,finish=1539271774938203014,duration=4715148
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06e31c00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a4b9d2b
travis_time:start:1a4b9d2b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:005d3a64
$ dmesg | grep -i kill
