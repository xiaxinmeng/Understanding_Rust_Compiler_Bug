plain
travis_time:end:1f6f2bf0:start=1541276932176436933,finish=1541276933245720461,duration=1069283528
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:07] .................................................................................................... 300/4988
[00:52:10] .................................................................................................... 400/4988
[00:52:13] .................................................................................................... 500/4988
[00:52:17] ........................i........................................................................... 600/4988
[00:52:21] ....................F............................................................................... 700/4988
[00:52:31] .....................................................................................iiiii.......... 900/4988
[00:52:35] .................................................................................................... 1000/4988
[00:52:37] .................................................................................................... 1100/4988
[00:52:39] .................................................................................................... 1200/4988
[00:52:39] .................................................................................................... 1200/4988
[00:52:42] .................................................................................................... 1300/4988
[00:52:44] .................................................................................................... 1400/4988
[00:52:47] ...........................................................................F.......i................ 1500/4988
[00:52:53] .................................................................................................... 1700/4988
[00:52:57] .................................................................................................... 1800/4988
[00:53:00] .............................................................................................i...... 1900/4988
[00:53:04] .................................................................................................... 2000/4988
---
[00:53:28] .................................................................................................... 2600/4988
[00:53:32] .................................................................................................... 2700/4988
[00:53:35] .................................................................................................... 2800/4988
[00:53:38] .................................................................................................... 2900/4988
[00:53:42] ..........................................FF........................................................ 3000/4988
[00:53:48] ..............................................................................i.i..ii............... 3200/4988
[00:53:52] .................................................................................................... 3300/4988
[00:53:56] .................................................................................................... 3400/4988
[00:53:59] ...................................................i.ii............................................. 3500/4988
---
[00:54:28] .................................................................................................... 4500/4988
[00:54:32] ..........................................................i......................................... 4600/4988
[00:54:35] .................................................................................................... 4700/4988
[00:54:38] .................................................................................................... 4800/4988
[00:54:41] ...............................................................F.................................... 4900/4988
xplanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":406,"byte_end":423,"line_start":14,"line_end":14,"column_start":17,"column_end":34,"is_primary":true,"text":[{"text":"    fn new() -> MustUseDeprecated { //~ warning: use of deprecated item","highlight_start":17,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:14:17\n   |\nLL |     fn new() -> MustUseDeprecated { //~ warning: use of deprecated item\n   |                 ^^^^^^^^^^^^^^^^^\n\n"}
[00:54:43] {"message":"use of deprecated item 'MustUseDeprecated'","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs","byte_start":470,"byte_end":487,"line_start":15,"line_end":15,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"        MustUseDeprecated {} //~ warning: use of deprecated item","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: use of deprecated item 'MustUseDeprecated'\n  --> /checkout/src/test/ui/conditional-compilation/cfg-attr-multi-true.rs:15:9\n   |\nLL |         MustUseDeprecated {} //~ warning: use of deprecated item\n   |         ^^^^^^^^^^^^^^^^^\n\n"}
[00:54:43] {"message":"unused ``MustUseDeprecated that must be untest.rs:3284:9
[00:54:43] 
[00:54:43] ---- [ui] ui/fn_must_use.rs stdout ----
[00:54:43] diff of stderr:
[00:54:43] 
[00:54:43] 
[00:54:43] - warning: unused return value of `need_to_use_this_value` that must be used
[00:54:43] + warning: unused return value of ``need_to_use_this_value that must be used
[00:54:43] 2   --> $DIR/fn_must_use.rs:65:5
[00:54:43] 3    |
[00:54:43] 4 LL |     need_to_use_this_value(); //~ WARN unused return value
[00:54:43] 11    |         ^^^^^^^^^^^^^^^
[00:54:43] 11    |         ^^^^^^^^^^^^^^^
[00:54:43] 12    = note: it's important
[00:54:43] 13 
[00:54:43] - warning: unused return value of `MyStruct::need_to_use_this_method_value` that must be used
[00:54:43] + warning: unused return value of ``MyStruct::need_to_use_this_method_value that must be used
[00:54:43] 15   --> $DIR/fn_must_use.rs:70:5
[00:54:43] 16    |
[00:54:43] 17 LL |     m.need_to_use_this_method_value(); //~ WARN unused return value
[00:54:43] 18    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:54:43] 19 
[00:54:43] 19 
[00:54:43] - warning: unused return value of `EvenNature::is_even` that must be used
[00:54:43] + warning: unused return value of ``EvenNature::is_even that must be used
[00:54:43] 21   --> $DIR/fn_must_use.rs:71:5
[00:54:43] 22    |
[00:54:43] 23 LL |     m.is_even(); // trait method!
[00:54:43] 25    |
[00:54:43] 25    |
[00:54:43] 26    = note: no side effects
[00:54:43] 27 
[00:54:43] - warning: unused return value of `MyStruct::need_to_use_this_associated_function_value` that must be used
[00:54:43] + warning: unused return value of ``MyStruct::need_to_use_this_associated_function_value that must be used
[00:54:43] 29   --> $DIR/fn_must_use.rs:74:5
[00:54:43] 30    |
[00:54:43] 31 LL |     MyStruct::need_to_use_this_associated_function_value();
[00:54:43] 32    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:54:43] 33 
[00:54:43] 33 
[00:54:43] - warning: unused return value of `std::cmp::PartialEq::eq` that must be used
[00:54:43] + warning: unused return value of ``std::cmp::PartialEq::eq that must be used
[00:54:43] 35   --> $DIR/fn_must_use.rs:80:5
[00:54:43] 36    |
[00:54:43] 37 LL |     2.eq(&3); //~ WARN unused return value
[00:54:43] 38    |     ^^^^^^^^^
[00:54:43] 39 
[00:54:43] 39 
[00:54:43] - warning: unused return value of `std::cmp::PartialEq::eq` that must be used
[00:54:43] + warning: unused return value of ``std::cmp::PartialEq::eq that must be used
[00:54:43] 41   --> $DIR/fn_must_use.rs:81:5
[00:54:43] 42    |
[00:54:43] 43 LL |     m.eq(&n); //~ WARN unused return value
[00:54:43] 
[00:54:43] The actual stderr differed from the expected stderr.
[00:54:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn_must_use/fn_must_use.stderr
[00:54:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn_must_use/fn_must_use.stderr
[00:54:43] To update references, rerun the tests and pass the `--bless` flag
[00:54:43] To only update this specific test, also pass `--test-args fn_must_use.rs`
[00:54:43] error: 1 errors occurred comparing output.
[00:54:43] status: exit code: 0
[00:54:43] status: exit code: 0
[00:54:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn_must_use.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn_must_use/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn_must_use/auxiliary" "-A" "unused"
[00:54:43] ------------------------------------------
[00:54:43] 
[00:54:43] ------------------------------------------
[00:54:43] stderr:
[00:54:43] stderr:
[00:54:43] ------------------------------------------
[00:54:43] {"message":"unused return value of ``need_to_use_this_value that must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/fn_must_use.rs","byte_start":1520,"byte_end":1545,"line_start":65,"line_end":65,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    need_to_use_this_value(); //~ WARN unused return value","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/fn_must_use.rs","byte_start":492,"byte_end":507,"line_start":13,"line_end":13,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![warn(unused_must_use)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"it's important","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: ighlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused comparison that must be used\n  --> /checkout/src/test/ui/fn_must_use.rs:84:5\n   |\nLL |     2 == 3; //~ WARN unused comparison\n   |     ^^^^^^\n\n"}
[00:54:43] {"message":"unused comparison that must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/fn_must_use.rs","byte_start":2166,"byte_end":2172,"line_start":85,"line_end":85,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    m == n; //~ WARN unused comparison","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused comparison that must be used\n  --> /checkout/src/test/ui/fn_must_use.rs:85:5\n   |\nLL |     m == n; //~ WARN unused comparison\n   |     ^^^^^^\n\n"}
[00:54:43] ------------------------------------------
[00:54:43] 
[00:54:43] thread '[ui] ui/fn_must_use.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:54:43] 
[00:54:43] 
[00:54:43] ---- [ui] ui/lint/must_use-trait.rs stdout ----
[00:54:43] diff of stderr:
[00:54:43] 
[00:54:43] - error: unused implementer of `Critical` that must be used
[00:54:43] + error: unused implementer of ``Critical that must be used
[00:54:43] 3    |
[00:54:43] 3    |
[00:54:43] 4 LL |     get_critical(); //~ ERROR unused implementer of `Critical` that must be used
--
[00:54:43] stderr:
[00:54:43] ------------------------------------------
[00:54:43] ------------------------------------------
[00:54:43] {"message":"unused return value of ``foo that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":145,"byte_end":151,"line_start":14,"line_end":14,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    foo(); //~ unused return value of `foo`","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":33,"byte_end":48,"line_start":3,"line_end":3,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![deny(unused_must_use)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused return value of ``foo that must be used\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:14:5\n   |\nLL |     foo(); //~ unused return value of `foo`\n   |     ^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:3:9\n   |\nLL | #![deny(unused_must_use)]\n   |         ^^^^^^^^^^^^^^^\n\n"}
[00:54:43] {"message":"unused return value of ``bar that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":190,"byte_euild/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused/unused-result/auxiliary" "-A" "unused"
[00:54:43] ------------------------------------------
[00:54:43] 
[00:54:43] ------------------------------------------
[00:54:43] stderr:
[00:54:43] stderr:
[00:54:43] ------------------------------------------
[00:54:43] {"message":"unused ``MustUse that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/unused/unused-result.rs","byte_start":934,"byte_end":951,"line_start":31,"line_end":31,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    foo::<MustUse>(); //~ ERROR: unused `MustUse` that must be used","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/unused/unused-result.rs","byte_start":512,"byte_end":527,"line_start":12,"line_end":12,"column_start":25,"column_end":40,"is_primary":true,"text":[{"text":"#![deny(unused_results, unused_must_use)]","highlight_start":25,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused ``MustUse that must be used\n  --> /checkout/src/test/ui/unused/unused-result.rs:31:5\n   |\nLL |     foo::<MustUse>(); //~ ERROR: unused `MustUse` that must be used\n   |     ^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined herspans":[{"file_name":"/checkout/src/test/ui/unused/unused-result.rs","byte_start":496,"byte_end":510,"line_start":12,"line_end":12,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"#![deny(unused_results, unused_must_use)]","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused result\n  --> /checkout/src/test/ui/unused/unused-result.rs:44:5\n   |\nLL |     foo::<isize>(); //~ ERROR: unused result\n   |     ^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/unused/unused-result.rs:12:9\n   |\nLL | #![deny(unused_results, unused_must_use)]\n   |         ^^^^^^^^^^^^^^\n\n"}
[00:54:43] {"message":"unused ``MustUse that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/unused/unused-result.rs","byte_start":1289,"byte_end":1306,"line_start":45,"line_end":45,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    foo::<MustUse>(); //~ ERROR: unused `MustUse` that must be used","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unused ``MustUse that must be used\n  --> /checkout/src/test/ui/unused/unused-result.rs:45:5\n   |\nLL |     foo::<MustUse>(); //~ ERROR: unused `MustUse` that must be used\n   |     ^^^^^^^^^^^^^^^^^\n\n"}
[00:54:43] {"message":"unused ``MustUseMsg that must be used","code":{"code":"unused_must_use","explanation":nulome tests failed', tools/compiletest/src/main.rs:503:22
[00:54:43] 
[00:54:43] 
[00:54:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:43] 
[00:54:43] 
[00:54:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:43] Build completed unsuccessfully in 0:03:51
[00:54:43] Build completed unsuccessfully in 0:03:51
[00:54:43] Makefile:58: recipe for target 'check' failed
[00:54:43] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bb43c30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
