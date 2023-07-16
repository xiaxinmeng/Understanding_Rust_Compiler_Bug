plain
[00:47:56] .................................................................................................... 2200/4573
[00:48:00] ...i................................................................................................ 2300/4573
[00:48:04] .................................................................................................... 2400/4573
[00:48:07] .................................................................................................... 2500/4573
[00:48:11] ................iiiiiiiii........................................................................... 2600/4573
[00:48:16] .................................................................................................... 2800/4573
[00:48:20] .................................................................................................... 2900/4573
[00:48:23] .....................................i.............................................................. 3000/4573
[00:48:25] .................................................................................................i.i 3100/4573
---
[00:50:49] .................................................................................................... 1000/2867
[00:51:00] .................................................................................................... 1100/2867
[00:51:07] .................................................................................................... 1200/2867
[00:51:16] .................................................................................................... 1300/2867
[00:51:27] ........................................................................i...F....................... 1400/2867
[00:51:48] .........................................i.......................................................... 1600/2867
[00:52:01] .................................................................................................... 1700/2867
[00:52:12] .................................................................................................... 1800/2867
[00:52:22] .................................................................i.................................. 1900/2867
[00:52:22] .................................................................i.................................. 1900/2867
[00:52:33] ...F...............................i................................................................ 2000/2867
[00:52:59] .................................................................................................ii. 2200/2867
[00:53:14] ....................................................................i....i.......................... 2300/2867
[00:53:27] ...........i........................................................................................ 2400/2867
[00:53:41] .................................................................................................... 2500/2867
[00:53:41] .................................................................................................... 2500/2867
[00:54:03] .................................................................................................... 2600/2867
[00:54:12] .................................................................................................... 2700/2867
[00:54:20] .................................................................................................... 2800/2867
[00:54:29] .......F...........................................................
[00:54:29] 
[00:54:29] ---- [run-pass] run-pass/issues/issue-32008.rs stdout ----
[00:54:29] normalized stderr:
[00:54:29] warning: unused arithmetic operation which must be used
[00:54:29] warning: unused arithmetic operation which must be used
[00:54:29]   --> $DIR/issue-32008.rs:30:5
[00:54:29]    |
[00:54:29] LL |     &*input + &local;
[00:54:29]    |
[00:54:29]    = note: #[warn(unused_must_use)] on by default
[00:54:29] 
[00:54:29] warning: unused arithmetic operation which must be used
[00:54:29] warning: unused arithmetic operation which must be used
[00:54:29]   --> $DIR/issue-32008.rs:34:5
[00:54:29]    |
[00:54:29] LL |     input + &local;
[00:54:29] 
[00:54:29] 
[00:54:29] 
[00:54:29] 
[00:54:29] 
[00:54:29] The actual stderr differed from the expected stderr.
[00:54:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/vel":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused arithmetic operation which must be used\n  --> /checkout/src/test/run-pass/issues/issue-32008.rs:30:5\n   |\nLL |     &*input + &local;\n   |     ^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_must_use)] on by default\n\n"}
[00:54:29] {"message":"unused arithmetic operation which must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-32008.rs","byte_start":1081,"byte_end":1095,"line_start":34,"line_end":34,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    input + &local;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused arithmetic operation which must be used\n  --> /checkout/src/test/run-pass/issues/issue-32008.rs:34:5\n   |\nLL |     input + &local;\n   |     ^^^^^^^^^^^^^^\n\n"}
[00:54:29] ------------------------------------------
[00:54:29] 
[00:54:29] thread '[run-pass] run-pass/issues/issue-32008.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:54:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:29] 
[00:54:29] ---- [run-pass] run-pass/never-type-rvalues.rs stdout ----
[00:54:29] normalized stderr:
[00:54:29] warning: unused unary operation which must be used
[00:54:29]   --> $DIR/never-type-rvalues.rs:21:5
[00:54:29]    |
[00:54:29] LL |     *x;
[00:54:29]    |
[00:54:29]    = note: #[warn(unused_must_use)] on by default
[00:54:29] 
[00:54:29] warning: unused unary operation which must be used
[00:54:29] warning: unused unary operation which must be used
[00:54:29]   --> $DIR/never-type-rvalues.rs:31:9
[00:54:29]    |
[00:54:29] LL |         *x;
[00:54:29] 
[00:54:29] 
[00:54:29] 
[00:54:29] 
[00:54:29] 
[00:54:29] The actual stderr differed from the expected stderr.
[00:54:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/never-type-rvalues/never-type-rvalues.stderr
[00:54:29] To update references, rerun the tests and pass the `--bless` flag
[00:54:29] To only update this specific test, also pass `--test-args never-type-rvalues.rs`
[00:54:29] error: 1 errors occurred comparing output.
[00:54:29] status: exit code: 0
[00:54:29] status: exit code: 0
[00:54:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/never-type-rvalues.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/never-type-rvalues/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/never-type-rvalues/auxiliary"
[00:54:29] ------------------------------------------
[00:54:29] 
[00:54:29] ------------------------------------------
[00:54:29] stderr:
[00:54:29] stderr:
[00:54:29] ------------------------------------------
[00:54:29] {"message":"unused unary operation which must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_nam9] {"message":"unused arithmetic operation which must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/unsized-locals/unsized-exprs.rs","byte_start":1132,"byte_end":1144,"line_start":46,"line_end":46,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    *afoo() + 42;","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_must_use)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused arithmetic operation which must be used\n  --> /checkout/src/test/run-pass/unsized-locals/unsized-exprs.rs:46:5\n   |\nLL |     *afoo() + 42;\n   |     ^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_must_use)] on by default\n\n"}
[00:54:29] ------------------------------------------
[00:54:29] 
[00:54:29] thread '[run-pass] run-pass/unsized-locals/unsized-exprs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:54:29] 
---
travis_time:end:0688f62f:start=1539033191911945787,finish=1539033191916201265,duration=4255478
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19bd0e1b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|
