plain
[00:46:19] .................................................................................................... 2700/4602
[00:46:22] .................................................................................................... 2800/4602
[00:46:26] .................................................................................................... 2900/4602
[00:46:29] ......................................................i............................................. 3000/4602
[00:46:32] ...........................................F........................................................ 3100/4602
[00:46:39] .................................................................................................... 3300/4602
[00:46:41] ..................................................................i................................. 3400/4602
[00:46:45] .................................................................................................... 3500/4602
[00:46:45] .................................................................................................... 3500/4602
[00:46:48] .......................................................................F............................ 3600/4602
[00:46:54] .................................................................................................... 3800/4602
[00:46:57] ..............i..................................................................................... 3900/4602
[00:47:01] .................................................................................................... 4000/4602
[00:47:04] .................................................................................................... 4100/4602
---
[00:47:19] ---- [ui] ui/match/match-unreachable-warning-with-diverging-discrim.rs stdout ----
[00:47:19] 
[00:47:19] error: ui test compiled successfully!
[00:47:19] status: exit code: 0
[00:47:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-unreachable-warning-with-diverging-discrim.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-unreachable-warning-with-diverging-discrim/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-unreachable-warning-with-diverging-discrim/auxiliary" "-A" "unused"
[00:47:19] ------------------------------------------
[00:47:19] 
[00:47:19] ------------------------------------------
[00:47:19] stderr:
---
[00:47:19] -   --> $DIR/expr_match.rs:19:5
[00:47:19] + error: unreachable statement
[00:47:19] +   --> $DIR/expr_match.rs:24:5
[00:47:19] 3    |
[00:47:19] - LL |     match {return} { } //~ ERROR unreachable
[00:47:19] -    |     ^^^^^^^^^^^^^^^^^^
[00:47:19] + LL |     println!("I am dead");
[00:47:19] 6    |
[00:47:19] 7 note: lint level defined here
[00:47:19] 8   --> $DIR/expr_match.rs:14:9
[00:47:19] 
[00:47:19] 
[00:47:19] 9    |
[00:47:19] 10 LL | #![deny(unreachable_code)]
[00:47:19] - 
[00:47:19] - error: unreachable statement
[00:47:19] -   --> $DIR/expr_match.rs:24:5
[00:47:19] -    |
[00:47:19] -    |
[00:47:19] - LL |     println!("I am dead");
[00:47:19] -    |
[00:47:19] 19    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:47:19] 20 
[00:47:19] 21 error: unreachable statement
---
[00:47:19] 31 
[00:47:19] 
[00:47:19] 
[00:47:19] The actual stderr differed from the expected stderr.
[00:47:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_match/expr_match.stderr
[00:47:19] To update references, rerun the tests and pass the `--bless` flag
[00:47:19] To only update this specific test, also pass `--test-args reachable/expr_match.rs`
[00:47:19] error: 1 errors occurred comparing output.
[00:47:19] status: exit code: 1
[00:47:19] status: exit code: 1
[00:47:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reachable/expr_match.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_match/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_match/auxiliary" "-A" "unused"
[00:47:19] ------------------------------------------
[00:47:19] 
[00:47:19] ------------------------------------------
[00:47:19] stderr:
[00:47:19] stderr:
[00:47:19] ------------------------------------------
[00:47:19] {"message":"unreachable statement","code":{"code":"unreachable_code","explanation":null},"level":"error","spans":[{"file_name":"<::std::macros::println macros>","byte_start":59,"byte_end":127,"line_start":2,"line_end":2,"column_start":1,"column_end":69,"is_primary":true,"text":[{"text":"{ $ crate :: io :: _print ( format_args_nl ! ( $ ( $ arg ) * ) ) ; } )","highlight_start":1,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/reachable/expr_match.rs","byte_start":761,"byte_end":783,"line_start":24,"line_end":24,"column_start":5,"column_end":27,"is_primary":false,"text":[{"text":"    println!(\"I am dead\");","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"println!","def_site_span":{"file_name":"<::std::macros::println macros>","byte_start":0,"byte_end":129,"line_start":1,"line_end":2,"column_start":1,"column_end":71,"is_primary":false,"text":[{"text":"(  ) => ( print ! ( \"\\n\" ) ) ; ( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":59},{"text":"{ $ crate :: io :: _print ( format_args_nl ! ( $ ( $ arg ) * ) ) ; } )","highlight_start":1,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/reachable/expr_match.rs","byte_start":554,"byte_end":570,"line_start":14,"line_end":14,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"#![deny(unreachable_code)]","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unreachable statement\n  --> /checkout/src/test/ui/reachable/expr_match.rs:24:5\n   |\nLL |     println!(\"I am dead\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/reachable/expr_match.rs:14:9\n   |\nLL | #![deny(unreachable_code)]\n   |         ^^^^^^^^^^^^^^^^\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:47:19] {"message":"unreachable statement","code":{"code":"unreachable_code","explanation":null},"level":"error","spans":[{"file_name":"<::std::macros::println macros>","byte_start":59,"byte_end":127,"line_start":2,"line_end":2,"column_start":1,"column_end":69,"is_primary":true,"text":[{"text":"{ $ crate :: io :: _print ( format_args_nl ! ( $ ( $ arg ) * ) ) ; } )","highlight_start":1,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/reachable/expr_match.rs","byte_start":982,"byte_end":1004,"line_start":35,"line_end":35,"column_start":5,"column_end":27,"is_primary":false,"text":[{"text":"    println!(\"I am dead\");","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"println!","def_site_span":{"file_name":"<::std::macros::println macros>","byte_start":0,"byte_end":129,"line_start":1,"line_end":2,"column_start":1,"column_end":71,"is_primary":false,"text":[{"text":"(  ) => ( print ! ( \"\\n\" ) ) ; ( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":59},{"text":"{ $ crate :: io :: _print ( format_args_nl ! ( $ ( $ arg ) * ) ) ; } )","highlight_start":1,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: unreachable statement\n  --> /checkout/src/test/ui/reachable/expr_match.rs:35:5\n   |\nLL |     println!(\"I am dead\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:47:19] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:47:19] ------------------------------------------
[00:47:19] 
[00:47:19] thread '[ui] ui/reachable/expr_match.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[00:47:19] 
---
[00:47:19] 
[00:47:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:47:19] 
[00:47:19] 
[00:47:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:19] 
[00:47:19] 
[00:47:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:19] Build completed unsuccessfully in 0:03:24
[00:47:19] Build completed unsuccessfully in 0:03:24
[00:47:19] make: *** [check] Error 1
[00:47:19] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09b68950
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:000dbba0:start=1539705734451917167,finish=1539705734456835035,duration=4917868
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22b1fb84
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0605fe14
travis_time:start:0605fe14
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2682c403
$ dmesg | grep -i kill
