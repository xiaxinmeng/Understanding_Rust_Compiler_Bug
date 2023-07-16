plain
travis_time:end:0192a176:start=1552418008739480772,finish=1552418082839979646,duration=74100498874
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:13:37] .................................................................................................... 3700/5463
[01:13:40] .......................................................................ii........................... 3800/5463
[01:13:43] .........................................................................................i.......... 3900/5463
[01:13:45] .................................................................................................... 4000/5463
[01:13:47] ............................................F..i.................................................... 4100/5463
[01:14:00] .................................................................................................... 4300/5463
[01:14:05] .................................................................................................... 4400/5463
[01:14:08] .................................................................................................... 4500/5463
[01:14:12] .................................................................................................... 4600/5463
---
[01:14:44] ---- [ui] ui/parser/macros-no-semicolon-items.rs stdout ----
[01:14:44] diff of stderr:
[01:14:44] 
[01:14:44] 5    |                 ^^
[01:14:44] 6 help: change the delimiters to curly braces
[01:14:44] 7    |
[01:14:44] - LL | macro_rules! foo {}  //~ ERROR semicolon
[01:14:44] + LL | macro_rules! foo {}
[01:14:44] 10 help: add a semicolon
[01:14:44] 11    |
[01:14:44] 
[01:14:44] 
[01:14:44] - LL | macro_rules! foo();  //~ ERROR semicolon
[01:14:44] + LL | macro_rules! foo();
[01:14:44] 14 
[01:14:44] 14 
[01:14:44] 15 error: macros that expand to items must be delimited with braces or followed by a semicolon
[01:14:44] 16   --> $DIR/macros-no-semicolon-items.rs:8:5
[01:14:44] 17    |
[01:14:44] 17    |
[01:14:44] - LL |   bar!( //~ ERROR semicolon
[01:14:44] + LL |   bar!(
[01:14:44] 20 LL | |     blah
[01:14:44] 21 LL | |     blah
[01:14:44] 
[01:14:44] 24    | |_^
[01:14:44] 24    | |_^
[01:14:44] 25 help: change the delimiters to curly braces
[01:14:44] 26    |
[01:14:44] - LL | bar! { //~ ERROR semicolon
[01:14:44] + LL | bar! {
[01:14:44] 28 LL |     blah
[01:14:44] 29 LL |     blah
[01:14:44] 30 LL |     blah
[01:14:44] 38 error: unexpected end of macro invocation
[01:14:44] 39   --> $DIR/macros-no-semicolon-items.rs:1:1
[01:14:44] 40    |
[01:14:44] 40    |
[01:14:44] - LL | macro_rules! foo()  //~ ERROR semicolon
[01:14:44] + LL | macro_rules! foo()
[01:14:44] 42    | ^^^^^^^^^^^^^^^^^^ missing tokens in macro arguments
[01:14:44] 44 error: aborting due to 3 previous errors
[01:14:44] 
[01:14:44] 
[01:14:44] The actual stderr differed from the expected stderr.
[01:14:44] The actual stderr differed from the expected stderr.
[01:14:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macros-no-semicolon-items/macros-no-semicolon-items.stderr
[01:14:44] To update references, rerun the tests and pass the `--bless` flag
[01:14:44] To only update this specific test, also pass `--test-args parser/macros-no-semicolon-items.rs`
[01:14:44] error: 1 errors occurred comparing output.
[01:14:44] status: exit code: 1
[01:14:44] status: exit code: 1
[01:14:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/macros-no-semicolon-items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macros-no-semicolon-items/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macros-no-semicolon-items/auxiliary" "-A" "unused"
[01:14:44] ------------------------------------------
[01:14:44] 
[01:14:44] ------------------------------------------
[01:14:44] stderr:
[01:14:44] stderr:
[01:14:44] ------------------------------------------
[01:14:44] {"message":"macros that expand to items must be delimited with braces or followed by a semicolon","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/macros-no-semicolon-items.rs","byte_start":16,"byte_end":18,"line_start":1,"line_end":1,"column_start":17,"column_end":19,"is_primary":true,"text":[{"text":"macro_rules! foo()  //~ ERROR semicolon","highlight_start":17,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"change the delimiters to curly braces","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/parser/macros-no-semicolon-items.rs","byte_start":16,"byte_end":17,"line_start":1,"line_end":1,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"macro_rules! foo()  //~ ERROR semicolon","highlight_start":17,"highlight_end":18}],"label":null,"suggested_replacement":" {","suggestion_applicability":"MaybeIncorrect","expansion":null},{"file_name":"/checkout/src/test/ui/parser/macros-no-semicolon-items.rs","byte_start":17,"byte_end":18,"line_start":1,"line_end":1,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"macro_rules! foo()  //~ ERROR semicolon","highlight_start":18,"highlight_end":19}],"label":null,"suggested_replacement":"}","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"add a semicolon","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/parser/macros-no-semicolon-items.rs","byte_start":18,"byte_end":18,"line_start":1,"line_end":1,"column_start":19,"column_end":19,"is_primary":true,"text":[{"text":"macro_rules! foo()  //~ ERROR semicolon","highlight_start":19,"highlight_end":19}],"label":null,"suggested_replacement":";","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: macros that expand to items must be delimited with braces or followed by a semicolon\n  --> /checkout/src/test/ui/parser/macros-no-semicolon-items.rs:1:17\n   |\nLL | macro_rules! foo()  //~ ERROR semicolon\n   |                 ^^\nhelp: change the delimiters to curly braces\n   |\nLL | macro_rules! foo {}  //~ ERROR semicolon\n   |                  ^^\nhelp: add a semicolon\n   |\nLL | macro_rules! foo();  //~ ERROR semicolon\n   |                   ^\n\n"}
[01:14:44] {"message":"macros that expand to items must be delimited with braces or followed by a semicolon","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/macros-no-semicolon-items.rs","byte_start":149,"byte_end":199,"line_start":8,"line_end":12,"column_start":5,"column_end":2,"is_primary":true,"text":[{"text":"bar!( //~ ERROR semicolon","highlight_start":5,"highlight_end":26},{"text":"    blah","highlight_start":1,"highllowed by a semicolon\n  --> /checkout/src/test/ui/parser/macros-no-semicolon-items.rs:8:5\n   |\nLL |   bar!( //~ ERROR semicolon\n   |  _____^\nLL | |     blah\nLL | |     blah\nLL | |     blah\nLL | | )\n   | |_^\nhelp: change the delimiters to curly braces\n   |\nLL | bar! { //~ ERROR semicolon\nLL |     blah\nLL |     blah\nLL |     blah\nLL | }\n   |\nhelp: add a semicolon\n   |\nLL | );\n   |  ^\n\n"}
[01:14:44] {"message":"unexpected end of macro invocation","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/macros-no-semicolon-items.rs","byte_start":0,"byte_end":18,"line_start":1,"line_end":1,"column_start":1,"column_end":19,"is_primary":true,"text":[{"text":"macro_rules! foo()  //~ ERROR semicolon","highlight_start":1,"highlight_end":19}],"label":"missing tokens in macro arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/ui/parser/macros-no-semicolon-items.rs:1:1\n   |\nLL | macro_rules! foo()  //~ ERROR semicolon\n   | ^^^^^^^^^^^^^^^^^^ missing tokens in macro arguments\n\n"}
[01:14:44] 
[01:14:44] ------------------------------------------
[01:14:44] 
[01:14:44] thread '[ui] ui/parser/macros-no-semicolon-items.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:14:44] 
[01:14:44] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:14:44] 
[01:14:44] 
[01:14:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:44] 
[01:14:44] 
[01:14:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:44] Build completed unsuccessfully in 0:04:23
[01:14:44] Build completed unsuccessfully in 0:04:23
[01:14:44] make: *** [check] Error 1
[01:14:44] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09d7ae11
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 20:29:37 UTC 2019
