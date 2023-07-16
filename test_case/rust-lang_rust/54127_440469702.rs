plain
travis_time:end:1e255d70:start=1542753533671964526,finish=1542753534741242685,duration=1069278159
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:56:48] 
[00:56:48] running 5043 tests
[00:56:51] .................................................................................................... 100/5043
[00:56:54] .................................................................................................... 200/5043
[00:56:58] .............................ii............................................ii....................ii. 300/5043
[00:57:01] ...............................................................................................iii.. 400/5043
[00:57:04] .....iiiiiiii.iii............................iii...........................................i........ 500/5043
[00:57:12] .................................................................................................... 700/5043
[00:57:19] ..................................................................................i...........i..... 800/5043
[00:57:23] .................................................................................................... 900/5043
[00:57:23] .................................................................................................... 900/5043
[00:57:27] .iiiii...................iiiiii..................................................................... 1000/5043
[00:57:29] ............................................................................iiiiiiii................ 1100/5043
[00:57:35] .................................................................................................... 1300/5043
[00:57:38] .................................................................................................... 1400/5043
[00:57:40] .................................i.................................................................. 1500/5043
[00:57:44] ..i.........ii.........................................................i............................ 1600/5043
---
[00:58:07] .................................................................................................... 2200/5043
[00:58:12] .................................................................................................... 2300/5043
[00:58:16] .................................................................................................... 2400/5043
[00:58:20] .................................................................................................... 2500/5043
[00:58:25] .....................................................................................iiiiiiiii...... 2600/5043
[00:58:33] ...................................................ii............................................... 2800/5043
[00:58:36] .................................................................................................... 2900/5043
[00:58:41] .................................................................................................... 3000/5043
[00:58:44] ...............................................i.................................................... 3100/5043
---
[01:00:26] .................................................................................................... 300/2886
[01:00:38] .................................................................................................... 400/2886
[01:00:48] .................................................................................................... 500/2886
[01:01:01] .................................................................................................... 600/2886
[01:01:17] .............F...................................................................................... 700/2886
[01:01:41] .................................................................................................... 900/2886
[01:01:57] .................................................................................................... 1000/2886
[01:02:12] .................................................................................................... 1100/2886
[01:02:22] .................................................................................................... 1200/2886
[01:02:22] .................................................................................................... 1200/2886
[01:02:33] .................................................................................................... 1300/2886
[01:02:47] .................................................................F............i..................... 1400/2886
[01:03:01] .................................................................................................... 1500/2886
[01:03:16] ......................................F........i......................F............................. 1600/2886
[01:03:44] .................................................................................................... 1800/2886
[01:03:58] ..........................................................................i......................... 1900/2886
[01:03:58] ..........................................................................i......................... 1900/2886
[01:04:18] ............F................................i...................................................... 2000/2886
[01:04:58] .................................................................................................... 2200/2886
[01:05:16] ........ii.....................................................................i....i............... 2300/2886
[01:05:32] .......................i............................................................................ 2400/2886
[01:05:44] .................................................................................................... 2500/2886
---
[01:06:54] failures:
[01:06:54] 
[01:06:54] ---- [run-pass] run-pass/drop/drop-uninhabited-enum.rs stdout ----
[01:06:54] normalized stderr:
[01:06:54] warning: functions with parameters of uninhabited types are uncallable
[01:06:54]    |
[01:06:54]    |
[01:06:54] LL | fn foo(x: Foo) { }
[01:06:54]    | ^^^^^^^-^^^^^^^^^^
[01:06:54]    |        this parameter has an uninhabited type
[01:06:54]    |
[01:06:54]    = note: #[warn(unreachable_code)] on by default
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] The actual stderr differed from the expected stderr.
[01:06:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-uninhabited-enum/drop-uninhabited-enum.stderr
[01:06:54] To update references, rerun the tests and pass the `--bless` flag
[01:06:54] To only update this specific test, also pass `--test-args drop/drop-uninhabited-enum.rs`
[01:06:54] error: 1 errors occurred comparing output.
[01:06:54] status: exit code: 0
[01:06:54] status: exit code: 0
[01:06:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-uninhabited-enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/drop-uninhabited-enum/auxiliary"
[01:06:54] ------------------------------------------
[01:06:54] 
[01:06:54] ------------------------------------------
[01:06:54] stderr:
[01:06:54] stderr:
[01:06:54] ------------------------------------------
[01:06:54] {"message":"functions with parameters of uninhabited types are uncallable","code":{"code":"unreachable_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs","byte_start":632,"byte_end":633,"line_start":22,"line_end":22,"column_start":8,"column_end":9,"is_primary":false,"text":[{"text":"fn foo(x: Foo) { }","highlight_start":8,"highlight_end":9}],"label":"this parameter has an uninhabited type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs","byte_start":625,"byte_end":643,"line_start":22,"line_end":22,"column_start":1,"column_end":19,"is_primary":true,"text":[{"text":"fn foo(x: Foo) { }","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unreachable_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: functions with parameters of uninhabited types are uncallable\n  --> /checkout/src/test/run-pass/drop/drop-uninhabited-enum.rs:22:1\n   |\nLL | fn foo(x: Foo) { }\n   | ^^^^^^^-^^^^^^^^^^\n   |        |\n   |        this parameter has an uninhabited type\n   |\n   = note: #[warn(unreachable_code)] on by default\n\n"}
[01:06:54] ------------------------------------------
[01:06:54] 
[01:06:54] thread '[run-pass] run-pass/drop/drop-uninhabited-enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:06:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:54] 
[01:06:54] ---- [run-pass] run-pass/issues/issue-3037.rs stdout ----
[01:06:54] normalized stderr:
[01:06:54] warning: functions with parameters of uninhabited types are uncallable
[01:06:54]    |
[01:06:54]    |
[01:06:54] LL |   fn what_to_string(x: what) -> String
[01:06:54]    |   ^                 - this parameter has an uninhabited type
[01:06:54]    |  _|
[01:06:54]    | |
[01:06:54] LL | | {
[01:06:54] LL | |     match x {
[01:06:54] LL | |     }
[01:06:54] LL | | }
[01:06:54]    |
[01:06:54]    = note: #[warn(unreachable_code)] on by default
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] The actual stderr differed from the expected stderr.
[01:06:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-3037/issue-3037.stderr
[01:06:54] To update references, rerun the tests and pass the `--bless` flag
[01:06:54] To only update this specific test, also pass `--test-args issues/issue-3037.rs`
[01:06:54] error: 1 errors occurred comparing output.
[01:06:54] status: exit code: 0
[01:06:54] status: exit code: 0
[01:06:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-3037.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-3037/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-3037/auxiliary"
[01:06:54] ------------------------------------------
[01:06:54] 
[01:06:54] ------------------------------------------
[01:06:54] stderr:
[01:06:54] stderr:
[01:06:54] ------------------------------------------
[01:06:54] {"message":"functions with parameters of uninhabited types are uncallable","code":{"code":"unreachable_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-3037.rs","byte_start":598,"byte_end":599,"line_start":18,"line_end":18,"column_start":19,"column_end":20,"is_primary":false,"text":[{"text":"fn what_to_string(x: what) -> String","highlight_start":19,"highlight_end":20}],"label":"this parameter has an uninhabited type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/issues/issue-3037.rs","byte_start":580,"byte_end":640,"line_start":18,"line_end":22,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn what_to_string(x: what) -> String","highlight_start":1,"highlight_end":37},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"    match x {","highlight_start":1,"highlight_end":14},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unreachable_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: functions with parameters of uninhabited types are uncallable\n  --> /checkout/src/test/run-pass/issues/issue-3037.rs:18:1\n   |\nLL |   fn what_to_string(x: what) -> String\n   |   ^                 - this parameter has an uninhabited type\n   |  _|\n   | |\nLL | | {\nLL | |     match x {\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = note: #[warn(unreachable_code)] on by default\n\n"}
[01:06:54] ------------------------------------------
[01:06:54] 
[01:06:54] thread '[run-pass] run-pass/issues/issue-3037.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:06:54] 
[01:06:54] 
[01:06:54] ---- [run-pass] run-pass/issues/issue-46855.rs stdout ----
[01:06:54] normalized stderr:
[01:06:54] warning: functions with parameters of uninhabited types are uncallable
[01:06:54]    |
[01:06:54]    |
[01:06:54] LL | fn foo(xs: [(Never, u32); 1]) -> u32 { xs[0].1 }
[01:06:54]    | ^^^^^^^--^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:54]    |        this parameter has an uninhabited type
[01:06:54]    |
[01:06:54]    = note: #[warn(unreachable_code)] on by default
[01:06:54] 
[01:06:54] 
[01:06:54] warning: functions with parameters of uninhabited types are uncallable
[01:06:54]    |
[01:06:54]    |
[01:06:54] LL | fn bar([(_, x)]: [(Never, u32); 1]) -> u32 { x }
[01:06:54]    |        |
[01:06:54]    |        this parameter has an uninhabited type
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] The actual stderr differed from the expected stderr.
[01:06:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-46855/issue-46855.stderr
[01:06:54] To update references, rerun the tests and pass the `--bless` flag
[01:06:54] To only update this specific test, also pass `--test-args issues/issue-46855.rs`
[01:06:54] error: 1 errors occurred comparing output.
[01:06:54] status: exit code: 0
[01:06:54] status: exit code: 0
[01:06:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-46855.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-46855/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-46855/auxiliary"
[01:06:54] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:06:54] ------------------------------------------
[01:06:54] 
[01:06:54] ------------------------------------------
[01:06:54] stderr:
[01:06:54] stderr:
[01:06:54] ------------------------------------------
[01:06:54] {"message":"functions with parameters of uninhabited types are uncallable","code":{"code":"unreachable_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-46855.rs","byte_start":637,"byte_end":639,"line_start":25,"line_end":25,"column_start":8,"column_end":10,"is_primary":false,"text":[{"text":"fn foo(xs: [(Never, u32); 1]) -> u32 { xs[0].1 }","highlight_start":8,"highlight_end":10}],"label":"this parameter has an uninhabited type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/issues/issue-46855.rs","byte_start":630,"byte_end":678,"line_start":25,"line_end":25,"column_start":1,"column_end":49,"is_primary":true,"text":[{"text":"fn foo(xs: [(Never, u32); 1]) -> u32 { xs[0].1 }","highlight_start":1,"highnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-50731.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-50731/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-50731/auxiliary"
[01:06:54] ------------------------------------------
[01:06:54] 
[01:06:54] ------------------------------------------
[01:06:54] stderr:
[01:06:54] stderr:
[01:06:54] ------------------------------------------
[01:06:54] {"message":"functions with parameters of uninhabited types are uncallable","code":{"code":"unreachable_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-50731.rs","byte_start":499,"byte_end":500,"line_start":13,"line_end":13,"column_start":8,"column_end":9,"is_primary":false,"text":[{"text":"fn foo(_: Result<(Void, u32), (Void, String)>) {}","highlight_start":8,"highlight_end":9}],"label":"this parameter has an uninhabited type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/issues/issue-50731.rs","byte_start":492,"byte_end":541,"line_start":13,"line_end":13,"column_start":1,"column_end":50,"is_primary":true,"text":[{"text":"fn foo(_: Result<(Void, u32), (Void, String)>) {}","highlight_start":1,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unreachable_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: functions with parameters of uninhabited types are uncallable\n  --> /checkout/src/test/run-pass/issues/issue-50731.rs:13:1\n   |\nLL | fn foo(_: Result<(Void, u32), (Void, String)>) {}\n   | ^^^^^^^-^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |        |\n   |        this parameter has an uninhabited type\n   |\n   = note: #[warn(unreachable_code)] on by default\n\n"}
[01:06:54] ------------------------------------------
[01:06:54] 
[01:06:54] thread '[run-pass] run-pass/issues/issue-50731.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:06:54] 
[01:06:54] 
[01:06:54] ---- [run-pass] run-pass/never-type-rvalues.rs stdout ----
[01:06:54] normalized stderr:
[01:06:54] warning: functions with parameters of uninhabited types are uncallable
[01:06:54]   --> $DIR/never-type-rvalues.rs:16:1
[01:06:54]    |
[01:06:54] LL |   fn never_direct(x: !) {
[01:06:54]    |   ^               - this parameter has an uninhabited type
[01:06:54]    |  _|
[01:06:54]    | |
[01:06:54] LL | |     x;
[01:06:54] LL | | }
[01:06:54]    |
[01:06:54]    = note: #[warn(unreachable_code)] on by default
[01:06:54] 
[01:06:54] 
[01:06:54] warning: functions with parameters of uninhabited types are uncallable
[01:06:54]   --> $DIR/never-type-rvalues.rs:20:1
[01:06:54]    |
[01:06:54] LL |   fn never_ref_pat(ref x: !) {
[01:06:54]    |   ^                ----- this parameter has an uninhabited type
[01:06:54]    |  _|
[01:06:54]    | |
[01:06:54] LL | |     *x;
[01:06:54] LL | | }
[01:06:54] 
[01:06:54] 
[01:06:54] warning: functions with parameters of uninhabited types are uncallable
[01:06:54]   --> $DIR/never-type-rvalues.rs:24:1
[01:06:54]    |
[01:06:54] LL |   fn never_ref(x: &!) {
[01:06:54]    |   ^            - this parameter has an uninhabited type
[01:06:54]    |  _|
[01:06:54]    | |
[01:06:54] LL | |     let &y = x;
[01:06:54] LL | |     y;
[01:06:54] LL | | }
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] 
[01:06:54] The actual stderr differed from the expected stderr.
[01:06:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/never-type-rvalues/never-type-rvalues.stderr
[01:06:54] To update references, rerun the tests and pass the `--bless` flag
[01:06:54] To only update this specific test, also pass `--test-args never-type-rvalues.rs`
[01:06:54] error: 1 errors occurred comparing output.
[01:06:54] status: exit code: 0
[01:06:54] status: exit code: 0
[01:06:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/never-type-rvalues.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/never-type-rvalues/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/never-type-rvalues/auxiliary"
[01:06:54] ------------------------------------------
[01:06:54] 
[01:06:54] ------------------------------------------
[01:06:54] stderr:
[01:06:54] stderr:
[01:06:54] ------------------------------------------
[01:06:54] {"message":"functions with parameters of uninhabited types are uncallable","code":{"code":"unreachable_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/never-type-rvalues.rs","byte_start":588,"byte_end":589,"line_start":16,"line_end":16,"column_start":17,"column_end":18,"is_primary":false,"text":[{"text":"fn never_direct(x: !) {","highlight_start":17,"highlight_end":18}],"label":"this parameter has an uninhabited type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/never-type-rvalues.rs","byte_start":572,"byte_end":604,"line_start":16,"line_end":18,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn never_direct(x: !) {","highlight_start":1,"highlight_end":24},{"text":"    x;","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unreachable_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: functions with parameters of uninhabited types are uncallable\n  --> /checkout/src/test/run-pass/never-type-rvalues.rs:16:1\n   |\nLL |   fn never_direct(x: !) {\n   |   ^               - this parameter has an uninhabited type\n   |  _|\n   | |\nLL | |     x;\nLL | | }\n   | |_^\n   |\n   = note: #[warn(unreachable_code)] on by default\n\n"}
[01:06:54] {"message":"functions with parameters of uninhabited types are uncallable","code":{"code":"unreachable_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/never-type-rvalues.rs","byte_start":623,"byte_end":628,"line_start":20,"line_end":20,"column_start":18,"column_end":23,"is_primary":false,"text":[{"text":"fn never_ref_pat(ref x: !) {","highlight_start":18,"highlight_end":23}],"label":"this parameter has an uninhabited type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/never-type-rvalues.rs","byte_start":606,"byte_end":644,"line_start":20,"line_end":22,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn never_ref_pat(ref x: !) {","highlight_start":1,"highlight_end":29},{"text":"    *x;","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: functions with parameters of uninhabited types are uncallable\n  --> /checkout/src/test/run-pass/never-type-rvalues.rs:20:1\n   |\nLL |   fn never_ref_pat(ref x: !) {\n   |   ^                ----- this parameter has an uninhabited type\n   |  _|\n   | |\nLL | |     *x;\nLL | | }\n   | |_^\n\n"}
[01:06:54] {"message":"functions with parameters of uninhabited types are uncallable","code":{"code":"unreachable_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/never-type-rvalues.rs","byte_start":659,"byte_end":660,"line_start":24,"line_end":24,"column_start":14,"coluTue, 20 Nov 2018 23:45:59 GMT
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0db8b5cb
