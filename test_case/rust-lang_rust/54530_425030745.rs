plain
[01:14:21] ....................................................................................................
[01:14:25] ..............................................................i.....................................
[01:14:28] ...............................................................................................i....
[01:14:33] ....................................................................................................
[01:14:37] ...........iiiiiiiii................................................................................
[01:14:44] ....................................................................................................
[01:14:50] ...............................................................................................i....
[01:14:53] ....................................................................................................
[01:14:57] .......................................................i.i..ii......................................
---
[01:15:43] ..........................................................................................ii........
[01:15:48] ....................................................................................................
[01:15:52] ....................................................................................................
[01:15:56] ....................................................................................................
[01:15:59] i.i..i.ii.ii.ii.............................i.......................................................
[01:15:59] test result: ok. 4218 passed; 0 failed; 87 ignored; 0 measured; 0 filtered out
[01:15:59] 
[01:15:59]  finished in 180.179
[01:15:59] travis_fold:end:test_ui_nll
---
[01:18:10] .............................................i..........................................i...........
[01:18:27] ..........i......i.....i............................................................................
[01:18:43] ....................................................................................................
[01:19:06] ....................................................................................................
[01:19:26] .....................................................................................F..............
[01:19:53] ....................................................................................................
[01:20:08] ....................................................................................................
[01:20:27] ....................................................................................................
[01:20:47] ..............i...............i.....................................................................
---
[01:22:30] ......................................................i..i.................i........................
[01:23:02] ....................i......................................................................i........
[01:23:18] ....................................................................................................
[01:23:40] ...................ii....................................................................i..........
[01:24:01] .......F...........................ii...............................................................
[01:25:12] ....................................................................................................
[01:25:29] ....................................................................................................
[01:25:44] ....................................................................................................
[01:26:05] ............................................i.............................................
[01:26:05] ............................................i.............................................
[01:26:05] failures:
[01:26:05] 
[01:26:05] ---- [run-pass] run-pass/issues/issue-14936.rs stdout ----
[01:26:05] normalized stderr:
[01:26:05] warning: unused macro definition
[01:26:05]   --> $DIR/issue-14936.rs:22:1
[01:26:05]    |
[01:26:05] LL | / macro_rules! demo {
[01:26:05] LL | |     ( $output_constraint:tt ) => {
[01:26:05] LL | |         {
[01:26:05] LL | |             let mut x: isize = 0;
[01:26:05] LL | |     }
[01:26:05] LL | | }
[01:26:05]    | |_^
[01:26:05]    |
[01:26:05]    |
[01:26:05]    = note: #[warn(unused_macros)] on by default
[01:26:05] 
[01:26:05] warning: type alias is never used: `History`
[01:26:05]    |
[01:26:05]    |
[01:26:05] LL | type History = Vec<&'static str>;
[01:26:05]    |
[01:26:05]    = note: #[warn(dead_code)] on by default
[01:26:05] 
[01:26:05] 
[01:26:05] warning: function is never used: `wrap`
[01:26:05]    |
[01:26:05]    |
[01:26:05] LL | fn wrap<A>(x:A, which: &'static str, history: &mut History) -> A {
[01:26:05] 
[01:26:05] 
[01:26:05] 
[01:26:05] 
[01:26:05] 
[01:26:05] The actual stderr differed from the expected stderr.
[01:26:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-14936/issue-14936.stderr
[01:26:05] To update references, rerun the tests and pass the `--bless` flag
[01:26:05] To only update this specific test, also pass `--test-args issues/issue-14936.rs`
[01:26:05] error: 1 errors occurred comparing output.
[01:26:05] status: exit code: 0
[01:26:05] status: exit code: 0
[01:26:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-14936.rs" "--target=arm-linux-androideabi" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-14936/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-14936/auxiliary"
[01:26:05] ------------------------------------------
[01:26:05] 
[01:26:05] ------------------------------------------
[01:26:05] stderr:
[01:26:05] stderr:
[01:26:05] ------------------------------------------
[01:26:05] {"message":"unused macro definition","code":{"code":"unused_macros","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-14936.rs","byte_start":639,"byte_end":1193,"line_start":22,"line_end":40,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"macro_rules! demo {","highlight_start":1,"highlight_end":20},{"text":"    ( $output_constraint:tt ) => {","highlight_start":1,"highlight_end":35},{"text":"        {","highlight_start":1,"highlight_end":10},{"text":"            let mut x: isize = 0;","highlight_start":1,"highlight_end":34},{"text":"            let y: isize = 1;","highlight_start":1,"highlight_end":30},{"text":"","highlight_start":1,"highlight_end":1},{"text":"            let mut history: History = vec![];","highlight_start":1,"highlight_end":47},{"text":"            unsafe {","highlight_start":1,"highlight_end":21},{"text":"                asm!(\"mov ($1), $0\"","highlight_start":1,"highlight_end":36},{"text":"                     : $output_constraint (*wrap(&mut x, \"out\", &mut history))","highlight_start":1,"highlight_end":79},{"text":"                     : \"r\"(&wrap(y, \"in\", &mut history))","highlight_start":1,"highlight_end":57},{"text":"                     :: \"volatile\");","highlight_start":1,"highlight_end":37},{"text":"            }","highlight_start":1,"highlight_end":14},{"text":"            assert_eq!((x,y), (1,1));","highlight_start":1,"highlight_end":38},{"text":"            let b: &[_] = &[\"out\", \"in\"];","highlight_start":1,"highlight_end":42},{"text":"            assert_eq!(history, b);","highlight_start":1,"highlight_end":36},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_macros)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused macro definition\n  --> /checkout/src/test/run-pass/issues/issue-14936.rs:22:1\n   |\nLL | / macro_rules! demo {\nLL | |     ( $output_constraint:tt ) => {\nLL | |         {\nLL | |             let mut x: isize = 0;\n...  |\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = note: #[warn(unused_macros)] on by default\n\n"}
[01:26:05] {"message":"type alias is never used: `History`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-14936.rs","byte_start":503,"byte_end":536,"line_start":15,"line_end":15,"column_start":1,"column_end":34,"is_primary":true,"text":[{"text":"type History = Vec<&'static str>;","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: type alias is never used: `History`\n  --> /checkout/src/test/run-pass/issues/issue-14936.rs:15:1\n   |\nLL | type History = Vec<&'static str>;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:26:05] {"message":"function is never used: `wrap`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-14936.rs","byte_start":538,"byte_end":602,"line_start":17,"line_end":17,"column_start":1,"column_end":65,"is_primary":true,"text":[{"text":"fn wrap<A>(x:A, which: &'static str, history: &mut History) -> A {","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: function is never used: `wrap`\n  --> /checkout/src/test/run-pass/issues/issue-14936.rs:17:1\n   |\nLL | fn wrap<A>(x:A, which: &'static str, history: &mut History) -> A {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:26:05] ------------------------------------------
[01:26:05] 
[01:26:05] thread '[run-pass] run-pass/issues/issue-14936.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:26:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:26:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:26:05] 
[01:26:05] ---- [run-pass] run-pass/simd/simd-target-feature-mixup.rs stdout ----
[01:26:05] normalized stderr:
[01:26:05] warning: unused variable: `level`
[01:26:05]    |
[01:26:05]    |
[01:26:05] LL |     pub fn main(level: &str) {}
[01:26:05]    |                 ^^^^^ help: consider using `_level` instead
[01:26:05]    = note: #[warn(unused_variables)] on by default
[01:26:05] 
[01:26:05] 
[01:26:05] 
[01:26:05] 
[01:26:05] 
[01:26:05] The actual stderr differed from the expected stderr.
[01:26:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd/simd-target-feature-mixup/simd-target-feature-mixup.stderr
[01:26:05] To update references, rerun the tests and pass the `--bless` flag
[01:26:05] To only update this specific test, also pass `--test-args simd/simd-target-feature-mixup.rs`
[01:26:05] error: 1 errors occurred comparing output.
[01:26:05] status: exit code: 0
[01:26:05] status: exit code: 0
[01:26:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/simd/simd-target-feature-mixup.rs" "--target=arm-linux-androideabi" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd/simd-target-feature-mixup/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd/simd-target-feature-mixup/auxiliary"
[01:26:05] ------------------------------------------
[01:26:05] 
[01:26:05] ------------------------------------------
[01:26:05] stderr:
[01:26:05] stderr:
[01:26:05] ------------------------------------------
[01:26:05] {"message":"unused variable: `level`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/simd/simd-target-feature-mixup.rs","byte_start":5543,"byte_end":5548,"line_start":192,"line_end":192,"column_start":17,"column_end":22,"is_primary":true,"text":[{"text":"    pub fn main(level: &str) {}","highlight_start":17,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_level` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/simd/simd-target-feature-mixup.rs","byte_start":5543,"byte_end":5548,"line_start":192,"line_end":192,"column_start":17,"column_end":22,"is_primary":true,"text":[{"text":"    pub fn main(level: &str) {}","highlight_start":17,"highlight_end":22}],"label":null,"suggested_replacement":"_level","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `level`\n  --> /checkout/src/test/run-pass/simd/simd-target-feature-mixup.rs:192:17\n   |\nLL |     pub fn main(level: &str) {}\n   |                 ^^^^^ help: consider using `_level` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:26:05] ------------------------------------------
[01:26:05] 
[01:26:05] thread '[run-pass] run-pass/simd/simd-target-feature-mixup.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[01:26:05] 
---
[01:26:05] 
[01:26:05] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:26:05] 
[01:26:05] 
[01:26:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "run-pass" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--quiet" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:26:05] 
[01:26:05] 
[01:26:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:26:05] Build completed unsuccessfully in 1:12:06
---
travis_time:end:002405a0:start=1538041532109425101,finish=1538041532125432136,duration=16007035
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1872f9bc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1cbaa93a
travis_time:start:1cbaa93a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13bfab0f
$ dmesg | grep -i kill
