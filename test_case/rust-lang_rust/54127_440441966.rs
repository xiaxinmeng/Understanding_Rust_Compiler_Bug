plain
travis_time:end:0e6b4aca:start=1542747433764019602,finish=1542747434755635536,duration=991615934
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:00] .............................i...................................................................... 600/5043
[00:51:04] .................................................................................................... 700/5043
[00:51:10] ..................................................................................i...........i..... 800/5043
[00:51:14] .................................................................................................... 900/5043
[00:51:17] .iiiii.............................................................................................. 1000/5043
[00:51:20] ......................................F............................................................. 1100/5043
[00:51:25] .................................................................................................... 1300/5043
[00:51:27] .................................................................................................... 1400/5043
[00:51:29] .................................................................................................... 1500/5043
[00:51:33] ..i....................................................................i............................ 1600/5043
---
[00:52:17] .................................................................................................... 2800/5043
[00:52:20] .................................................................................................... 2900/5043
[00:52:24] .................................................................................................... 3000/5043
[00:52:27] ...............................................i.................................................... 3100/5043
[00:52:31] ....................F............................................................................... 3200/5043
[00:52:38] .................................................................................................... 3400/5043
[00:52:41] ...........................................................................................ii....... 3500/5043
[00:52:44] .................................................................................................... 3600/5043
[00:52:45] .........i.......................................................................................... 3700/5043
[00:52:45] .........i.......................................................................................... 3700/5043
[00:52:46] .................................................................i.................................. 3800/5043
[00:52:48] .................................................................................................... 3900/5043
[00:52:52] ..............................................................................F..................... 4000/5043
[00:52:58] .................................................................................................... 4200/5043
[00:53:02] .........................i.......................................................................... 4300/5043
[00:53:07] .................................................................................................... 4400/5043
[00:53:10] .................................................................................................... 4500/5043
[00:53:10] .................................................................................................... 4500/5043
[00:53:14] .................................................................................................... 4600/5043
[00:53:18] .........i.......................................................................................... 4700/5043
[00:53:21] .................................................................................................... 4800/5043
[00:53:24] ...........................................................................F........................ 4900/5043
[00:53:28] - error: unreachable statement
[00:53:28] -   --> $DIR/match-no-arms-unreachable-after.rs:18:5
[00:53:28] -   --> $DIR/match-no-arms-unreachable-after.rs:18:5
[00:53:28] + error: functions with parameters of uninhabited types are uncallable
[00:53:28] +   --> $DIR/match-no-arms-unreachable-after.rs:16:1
[00:53:28] 3    |
[00:53:28] - LL |     let x = 2; //~ ERROR unreachable
[00:53:28] -    |     ^^^^^^^^^^
[00:53:28] + LL |   fn foo(v: Void) {
[00:53:28] +    |   ^      - this parameter has an uninhabited type
[00:53:28] +    |  _|
[00:53:28] +    | |
[00:53:28] + LL | |     match v { }
[00:53:28] + LL | |     let x = 2; //~ ERROR unreachable
[00:53:28] + LL | | }
[00:53:28] 6    |
[00:53:28] 7 note: lint level defined here
[00:53:28] 8   --> $DIR/match-no-arms-unreachable-after.rs:12:9
[00:53:28] 
[00:53:28] 
[00:53:28] 10 LL | #![deny(unreachable_code)]
[00:53:28] 12 
[00:53:28] - error: aborting due to previous error
[00:53:28] + error: unreachable statement
[00:53:28] +   --> $DIR/match-no-arms-unreachable-after.rs:18:5
[00:53:28] +   --> $DIR/match-no-arms-unreachable-after.rs:18:5
[00:53:28] +    |
[00:53:28] + LL |     let x = 2; //~ ERROR unreachable
[00:53:28] + 
[00:53:28] + error: aborting due to 2 previous errors
[00:53:28] 14 
[00:53:28] 15 
[00:53:28] 15 
[00:53:28] 
[00:53:28] 
[00:53:28] The actual stderr differed from the expected stderr.
[00:53:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-no-arms-unreachable-after/match-no-arms-unreachable-after.stderr
[00:53:28] To update references, rerun the tests and pass the `--bless` flag
[00:53:28] To only update this specERROR unreachable\n   |     ^^^^^^^^^^^\n\n"}
[00:53:28] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:53:28] ------------------------------------------
[00:53:28] 
[00:53:28] thread '[ui] ui/reachable/expr_call.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:53:28] 
[00:53:28] 
[00:53:28] ---- [ui] ui/unreachable/unwarned-match-on-never.rs stdout ----
[00:53:28] diff of stderr:
[00:53:28] 
[00:53:28] - error: unreachable expression
[00:53:28] -   --> $DIR/unwarned-match-on-never.rs:10:5
[00:53:28] + error: functions with parameters of uninhabited types are uncallable
[00:53:28] +   --> $DIR/unwarned-match-on-never.rs:6:1
[00:53:28] 3    |
[00:53:28] - LL |     match x {} //~ ERROR unreachable expression
[00:53:28] -    |     ^^^^^^^^^^
[00:53:28] + LL |   fn foo(x: !) -> bool {
[00:53:28] +    |   ^      - this parameter has an uninhabited type
[00:53:28] +    |  _|
[00:53:28] +    | |
[00:53:28] + LL | |     // Explicit matches on the never type are unwarned.
[00:53:28] + LL | |     match x {}
[00:53:28] + LL | |     // But matches in unreachable code are warned.
[00:53:28] + LL | |     match x {} //~ ERROR unreachable expression
[00:53:28] + LL | | }
[00:53:28] 6    |
[00:53:28] 7 note: lint level defined here
[00:53:28] 8   --> $DIR/unwarned-match-on-never.rs:1:9
[00:53:28] 
[00:53:28] 
[00:53:28] 10 LL | #![deny(unreachable_code)]
[00:53:28] 12 
[00:53:28] + error: unreachable expression
[00:53:28] +   --> $DIR/unwarned-match-on-never.rs:10:5
[00:53:28] +    |
[00:53:28] +    |
[00:53:28] + LL |     match x {} //~ ERROR unreachable expression
[00:53:28] + 
[00:53:28] 13 error: unreachable arm
[00:53:28] 14   --> $DIR/unwarned-match-on-never.rs:15:15
[00:53:28] 15    |
[00:53:28] 15    |
[00:53:28] 
[00:53:28] 24 LL | |     }
[00:53:28] 26 
[00:53:28] - error: aborting due to 3 previous errors
[00:53:28] + error: aborting due to 4 previous errors
[00:53:28] 28 
[00:53:28] 28 
[00:53:28] 29 
[00:53:28] 
[00:53:28] 
[00:53:28] The actual stderr differed from the expected stderr.
[00:53:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unreachable/unwarned-match-on-never/unwarned-match-on-never.stderr
[00:53:28] To update references, rerun the tests and pass the `--bless` flag
[00:53:28] To only update this specific test, also pass `--test-args unreachable/unwarned-match-on-never.rs`
[00:53:28] error: 1 errors occurred comparing output.
[00:53:28] status: exit code: 1
[00:53:28] status: exit code: 1
[00:53:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unreachable/unwarned-match-on-never.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unreachable/unwarned-match-on-never/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unreachable/unwarned-match-on-never/auxiliary" "-A" "unused"
[00:53:28] ------------------------------------------
[00:53:28] 
[00:53:28] ------------------------------------------
[00:53:28] stderr:
[00:53:28] stderr:
[00:53:28] ------------------------------------------
[00:53:28] {"message":"functions with parameters of uninhabited types are uncallable","code":{"code":"unreachable_code","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/unreachable/unwarned-match-on-never.rs","byte_start":81,"byte_end":82,"line_start":6,"line_end":6,"column_start":8,"column_end":9,"is_primary":false,"text":[{"text":"fn foo(x: !) -> bool {","highlight_start":8,"highlight_end":9}],"label":"this parameter has an uninhabited type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/unreachable/unwarned-match-on-never.rs","byte_start":74,"byte_end":268,"line_start":6,"line_end":11,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn foo(x: !) -> bool {","highlight_start":1,"highlight_end":23},{"text":"    // Explicit matches on the never type are unwarned.","highlight_start":1,"highlight_end":56},{"text":"    match x {}","highlight_start":1,"highlight_end":15},{"text":"    // But matches in unreachable code are warned.","highlight_start":1,"highlight_end":51},{"text":"    match x {} //~ ERROR unreachable expression","highlight_start":1,"highlight_end":48},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name"nreachable expression\nLL | |         () => (),\nLL | |     }\n   | |_____^\n\n"}
[00:53:28] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:53:28] ------------------------------------------
[00:53:28] 
[00:53:28] thread '[ui] ui/unreachable/unwarned-match-on-never.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:53:28] 
---
[00:53:28] 
[00:53:28] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:53:28] 
[00:53:28] 
[00:53:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/
