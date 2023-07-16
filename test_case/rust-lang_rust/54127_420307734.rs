plain
[00:46:03] ....................................................................................................
[00:46:06] ....................................................................................................
[00:46:09] ....................................................................................................
[00:46:12] ...............................................................................i....................
[00:46:15] ...........................................F........................................................
[00:46:21] ....................................................................................................
[00:46:24] .........................................i..........................................................
[00:46:27] ....................................................................................................
[00:46:27] ....................................................................................................
[00:46:30] .....................................F..............................................................
[00:46:35] ..........................................................................i.........................
[00:46:43] ....................................................................................................
[00:46:50] ....................................................................................................
[00:46:57] ....................................................................................................
---
[00:51:16] diff of stderr:
[00:51:16] 
[00:51:16] - error: unreachable statement
[00:51:16] -   --> $DIR/match-no-arms-unreachable-after.rs:18:5
[00:51:16] + error: functions with parameters of uninhabited types are uncallable
[00:51:16] +   --> $DIR/match-no-arms-unreachable-after.rs:16:1
[00:51:16] 3    |
[00:51:16] - LL |     let x = 2; //~ ERROR unreachable
[00:51:16] -    |     ^^^^^^^^^^
[00:51:16] + LL |   fn foo(v: Void) {
[00:51:16] +    |   ^      - this parameter has an uninhabited type
[00:51:16] +    |  _|
[00:51:16] +    | |
[00:51:16] + LL | |     match v { }
[00:51:16] + LL | |     let x = 2; //~ ERROR unreachable
[00:51:16] + LL | | }
[00:51:16] 6    |
[00:51:16] 7 note: lint level defined here
[00:51:16] 8   --> $DIR/match-no-arms-unreachable-after.rs:12:9
[00:51:16] 
[00:51:16] 
[00:51:16] 10 LL | #![deny(unreachable_code)]
[00:51:16] 12 
[00:51:16] - error: aborting due to previous error
[00:51:16] + error: unreachable statement
[00:51:16] +   --> $DIR/match-no-arms-unreachable-after.rs:18:5
[00:51:16] +   --> $DIR/match-no-arms-unreachable-after.rs:18:5
[00:51:16] +    |
[00:51:16] + LL |     let x = 2; //~ ERROR unreachable
[00:51:16] + 
[00:51:16] + error: aborting due to 2 previous errors
[00:51:16] 14 
[00:51:16] 15 
[00:51:16] 15 
[00:51:16] 
[00:51:16] 
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-no-arms-unreachable-after/match-no-arms-unreachable-after.stderr
[00:51:16] To update references, rerun the tests and pass the `--bless` flag
[00:51:16] To only update this specific test, also pass `--test-args match/match-no-arms-unreachable-after.rs`
[00:51:16] error: 1 errors occurred comparing output.
[00:51:16] status: exit code: 1
[00:51:16] status: exit code: 1
[00:51:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-no-arms-unreachable-after.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-no-arms-unreachable-after/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/chece_start":12,"line_end":12,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"#![deny(unreachable_code)]","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: functions with parameters of uninhabited types are uncallable\n  --> /checkout/src/test/ui/match/match-no-arms-unreachable-after.rs:16:1\n   |\nLL |   fn foo(v: Void) {\n   |   ^      - this parameter has an uninhabited type\n   |  _|\n   | |\nLL | |     match v { }\nLL | |     let x = 2; //~ ERROR unreachable\nLL | | }\n   | |_^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/match/match-no-arms-unreachable-after.rs:12:9\n   |\nLL | #![deny(unreachable_code)]\n   |         ^^^^^^^^^^^^^^^^\n\n"}
[00:51:16] {"message":"unreachable statement","code":{"code":"unreachable_code","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/match/match-no-arms-unreachable-after.rs","byte_start":568,"byte_end":578,"line_start":18,"line_end":18,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    let x = 2; //~ ERROR unreachable","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unreachable statement\n  --> /checkout/src/test/ui/match/match-no-arms-unreachable-after.rs:18:5\n   |\nLL |     let x = 2; //~ ERROR unreachable\n   |     ^^^^^^^^^^\n\n"}
[00:51:16] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] thread '[ui] ui/match/match-no-arms-unreachable-after.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:51:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:16] 
[00:51:16] ---- [ui] ui/reachable/expr_call.rs stdout ----
[00:51:16] diff of stderr:
[00:51:16] 
[00:51:16] - error: unreachable expression
[00:51:16] -   --> $DIR/expr_call.rs:22:17
[00:51:16] + error: functions with parameters of uninhabited types are uncallable
[00:51:16] +   --> $DIR/expr_call.rs:16:1
[00:51:16] 3    |
[00:51:16] - LL |     foo(return, 22); //~ ERROR unreachable
[00:51:16] -    |                 ^^
[00:51:16] + LL | fn foo(x: !, y: usize) { }
[00:51:16] +    | ^^^^^^^-^^^^^^^^^^^^^^^^^^
[00:51:16] +    |        this parameter has an uninhabited type
[00:51:16] 6    |
[00:51:16] 7 note: lint level defined here
[00:51:16] 8   --> $DIR/expr_call.rs:14:9
[00:51:16] 8   --> $DIR/expr_call.rs:14:9
[00:51:16] 
[00:51:16] 10 LL | #![deny(unreachable_code)]
[00:51:16] 12 
[00:51:16] 12 
[00:51:16] + error: functions with parameters of uninhabited types are uncallable
[00:51:16] +   --> $DIR/expr_call.rs:18:1
[00:51:16] +    |
[00:51:16] + LL | fn bar(x: !) { }
[00:51:16] +    | ^^^^^^^-^^^^^^^^
[00:51:16] +    |        this parameter has an uninhabited type
[00:51:16] + 
[00:51:16] 13 error: unreachable expression
[00:51:16] +   --> $DIR/expr_call.rs:22:17
[00:51:16] +   --> $DIR/expr_call.rs:22:17
[00:51:16] +    |
[00:51:16] + LL |     foo(return, 22); //~ ERROR unreachable
[00:51:16] + 
[00:51:16] + error: unreachable expression
[00:51:16] 14   --> $DIR/expr_call.rs:27:5
[00:51:16] 15    |
[00:51:16] 15    |
[00:51:16] 16 LL |     bar(return); //~ ERROR unreachable
[00:51:16] 17    |     ^^^^^^^^^^^
[00:51:16] 18 
[00:51:16] - error: aborting due to 2 previous errors
[00:51:16] + error: aborting due to 4 previous errors
[00:51:16] + error: aborting due to 4 previous errors
[00:51:16] 20 
[00:51:16] 21 
[00:51:16] 
[00:51:16] 
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_call/expr_call.stderr
[00:51:16] To update references, rerun the tests and pass the `--bless` flag
[00:51:16] To only update this specific test, also pass `--test-args reachable/expr_call.rs`
[00:51:16] error: 1 errors occurred comparing output.
[00:51:16] status: exit code: 1
[00:51:16] status: exit code: 1
[00:51:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reachable/expr_call.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_call/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_call/auxiliary" "-A" "unused"
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] ------------------------------------------
[00:51:16] stderr:
[00:51:16] stderr:
n   |        this parameter has an uninhabited type\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/reachable/expr_call.rs:14:9\n   |\nLL | #![deny(unreachable_code)]\n   |         ^^^^^^^^^^^^^^^^\n\n"}
[00:51:16] {"message":"functions with parameters of uninhabited types are uncallable","code":{"code":"unreachable_code","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/reachable/expr_call.rs","byte_start":632,"byte_end":633,"line_start":18,"line_end":18,"column_start":8,"column_end":9,"is_primary":false,"text":[{"text":"fn bar(x: !) { }","highlight_start":8,"highlight_end":9}],"label":"this parameter has an uninhabited type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/reachable/expr_call.rs","byte_start":625,"byte_end":641,"line_start":18,"line_end":18,"column_start":1,"column_end":17,"is_primary":true,"text":[{"text":"fn bar(x: !) { }","highlight_start":1,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: functions with parameters of uninhabited types are uncallable\n  --> /checkout/src/test/ui/reachable/expr_call.rs:18:1\n   |\nLL | fn bar(x: !) { }\n   | ^^^^^^^-^^^^^^^^\n   |        |\n   |        this parameter has an uninhabited type\n\n"}
[00:51:16] {"message":"unreachable expression","code":{"code":"unreachable_code","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/reachable/expr_call.rs","byte_start":700,"byte_end":702,"line_start":22,"line_end":22,"column_start":17,"column_end":19,"is_primary":true,"text":[{"text":"    foo(return, 22); //~ ERROR unreachable","highlight_start":17,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unreachable expression\n  --> /checkout/src/test/ui/reachable/expr_call.rs:22:17\n   |\nLL |     foo(return, 22); //~ ERROR unreachable\n   |                 ^^\n\n"}
[00:51:16] {"message":"unreachable expression","code":{"code":"unreachable_code","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/reachable/expr_call.rs","byte_start":775,"byte_end":786,"line_start":27,"line_end":27,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    bar(return); //~ ERROR unreachable","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unreachable expression\n  --> /checkout/src/test/ui/reachable/expr_call.rs:27:5\n   |\nLL |     bar(return); //~ ERROR unreachable\n   |     ^^^^^^^^^^^\n\n"}
[00:51:16] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] thread '[ui] ui/reachable/expr_call.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:51:16] 
[00:51:16] 
[00:51:16] 
[00:51:16] failures:
[00:51:16]     [ui] ui/match/match-no-arms-unreachable-after.rs
[00:51:16]     [ui] ui/reachable/expr_call.rs
[00:51:107:25
[00:51:16] Makefile:58: recipe for target 'check' failed
[00:51:16] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08ef0314
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:10a74ede:start=1536678428405569714,finish=1536678428432075840,duration=26506126
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02eb197c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07388f79
$ dmesg | grep -i kill
