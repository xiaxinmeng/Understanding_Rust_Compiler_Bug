plain
travis_time:end:1eff35cd:start=1543346745266454500,finish=1543346747630103427,duration=2363648927
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:23] .................................................................................................... 2700/5062
[00:46:26] .................................................................................................... 2800/5062
[00:46:29] .................................................................................................... 2900/5062
[00:46:32] .................................................................................................... 3000/5062
[00:46:36] ............................................................F.......i............................... 3100/5062
[00:46:42] ...............................ii..i..ii............................................................ 3300/5062
[00:46:46] .................................................................................................... 3400/5062
[00:46:49] .................................................................................................... 3500/5062
[00:46:52] ............ii...................................................................................... 3600/5062
---
[00:47:34] 43 error: unexpected end of macro invocation
[00:47:34] -   --> $DIR/macro-at-most-once-rep-2018.rs:41:15
[00:47:34] +   --> $DIR/macro-at-most-once-rep-2018.rs:39:15
[00:47:34] 45    |
[00:47:34] 46 LL | macro_rules! barplus {
[00:47:34] 47    | -------------------- when calling this macro
[00:47:34] 77    |     ^^^^^^^^^^^ missing tokens in macro arguments
[00:47:34] 78 
[00:47:34] 79 error: unexpected end of macro invocation
[00:47:34] -   --> $DIR/macro-at-most-once-rep-2018.rs:46:14
[00:47:34] -   --> $DIR/macro-at-most-once-rep-2018.rs:46:14
[00:47:34] +   --> $DIR/macro-at-most-once-rep-2018.rs:46:15
[00:47:34] 81    |
[00:47:34] 82 LL | macro_rules! barstar {
[00:47:34] 83    | -------------------- when calling this macro
[00:47:34] 
[00:47:34] The actual stderr differed from the expected stderr.
[00:47:34] The actual stderr differed from the expected stderr.
[00:47:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-at-most-once-rep-2018/macro-at-most-once-rep-2018.stderr
[00:47:34] To update references, rerun the tests and pass the `--bless` flag
[00:47:34] To only update this specific test, also pass `--test-args macros/macro-at-most-once-rep-2018.rs`
[00:47:34] error: 1 errors occurred comparing output.
[00:47:34] status: exit code: 1
[00:47:34] status: exit code: 1
[00:47:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-at-most-once-rep-2018/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-at-most-once-rep-2018/auxiliary" "-A" "unused"
[00:47:34] ------------------------------------------
[00:47:34] 
[00:47:34] ------------------------------------------
[00:47:34] stderr:
[00:47:34] stderr:
[00:47:34] ------------------------------------------
[00:47:34] {"message":"the `?` macro repetition operator does not take a separator","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":634,"byte_end":635,"line_start":20,"line_end":20,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"    ($(a),?) => {}; //~ERROR the `?` macro repetition operator","highlight_start":10,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the `?` macro repetition operator does not take a separator\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs:20:10\n   |\nLL |     ($(a),?) => {}; //~ERROR the `?` macro repetition operator\n   |          ^\n\n"}
[00:47:34] {"message":"no rules expected the token `?`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":890,"byte_end":891,"line_start":34,"line_end":34,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    foo!(a?); //~ ERROR no rules expected the token `?`","highlight_start":11,"highlight_end":12}],"label":"no rules expected this token in macro call","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":565,"byte_end":581,"line_start":15,"line_end":15,"column_start":1,"column_end":17,"is_primary":false,"text":[{"text":"macro_rules! foo {","highlight_start":1,"highlight_end":17}],"label":"when calling this macro","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `?`\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs:34:11\n   |\nLL | macro_rules! foo {\n   | ---------------- when lumn_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    foo!(a?a?a); //~ ERROR no rules expected the token `?`","highlight_start":11,"highlight_end":12}],"label":"no rules expected this token in macro call","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":565,"byte_end":581,"line_start":15,"line_end":15,"column_start":1,"column_end":17,"is_primary":false,"text":[{"text":"macro_rules! foo {","highlight_start":1,"highlight_end":17}],"label":"when calling this macro","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `?`\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs:36:11\n   |\nLL | macro_rules! foo {\n   | ---------------- when calling this macro\n...\nLL |     foo!(a?a?a); //~ ERROR no rules expected the token `?`\n   |           ^ no rules expected this token in macro call\n\n"}
[00:47:34] {"message":"unexpected end of macro invocation","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":1057,"byte_end":1068,"line_start":38,"line_end":38,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    barplus!(); //~ERROR unexpected end of macro invocation","highlight_start":5,"highlight_end":16}],"label":"missing tokens in macro arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":691,"byte_end":711,"line_start":23,"line_end":23,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"macro_rules! barplus {","highlight_start":1,"highlight_end":21}],"label":"when calling this macro","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs:38:5\n   |\nLL | macro_rules! barplus {\n   | -------------------- when calling this macro\n...\nLL |     barplus!(); //~ERROR unexpected end of macro invocation\n   |     ^^^^^^^^^^^ missing tokens in macro arguments\n\n"}
[00:47:34] {"message":"unexpected end of macro invocation","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":1127,"byte_end":1127,"line_start":39,"line_end":39,"column_start":15,"column_end":15,"is_primary":true,"text":[{"text":"    barplus!(a); //~ERROR unexpected end of macro invocation","highlight_start":15,"highlight_end":15}],"label":"missing tokens in macro arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":691,"byte_end":711,"line_start":23,"line_end":23,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"macro_rules! barplus {","highlight_start":1,"highlight_end":21}],"label":"when calling this macro","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checko,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":765,"byte_end":785,"line_start":27,"line_end":27,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"macro_rules! barstar {","highlight_start":1,"highlight_end":21}],"label":"when calling this macro","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs:45:5\n   |\nLL | macro_rules! barstar {\n   | -------------------- when calling this macro\n...\nLL |     barstar!(); //~ERROR unexpected end of macro invocation\n   |     ^^^^^^^^^^^ missing tokens in macro arguments\n\n"}
[00:47:34] {"message":"unexpected end of macro invocation","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":1405,"byte_end":1405,"line_start":46,"line_end":46,"column_start":15,"column_end":15,"is_primary":true,"text":[{"text":"    barstar!(a); //~ERROR unexpected end of macro invocation","highlight_start":15,"highlight_end":15}],"label":"missing tokens in macro arguments","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":765,"byte_end":785,"line_start":27,"line_end":27,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"macro_rules! barstar {","highlight_start":1,"highlight_end":21}],"label":"when calling this macro","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs:46:15\n   |\nLL | macro_rules! barstar {\n   | -------------------- when calling this macro\n...\nLL |     barstar!(a); //~ERROR unexpected end of macro invocation\n   |               ^ missing tokens in macro arguments\n\n"}
[00:47:34] {"message":"no rules expected the token `?`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":1466,"byte_end":1467,"line_start":47,"line_end":47,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"    barstar!(a?); //~ ERROR no rules expected the token `?`","highlight_start":15,"highlight_end":16}],"label":"no rules expected this token in macro call","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":765,"byte_end":785,"line_start":27,"line_end":27,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"macro_rules! barstar {","highlight_start":1,"highlight_end":21}],"label":"when calling this macro","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `?`\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs:47:15\n   |\nLL | macro_rules! barstar {\n   | -------------------- when calling this macro\n...\nLL |     barstar!(a?); //~ ERROR no rules expected the token `?`\n   |               ^ no rules expected this token in macro call\n\n"}
[00:47:34] {"message":"no rules expected the token `?`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":1526,"byte_end":1527,"line_start":48,"line_end":48,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"    barstar!(a?a); //~ ERROR no rules expected the token `?`","highlight_start":15,"highlight_end":16}],"label":"no rules expected this token in macro call","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs","byte_start":765,"byte_end":785,"line_start":27,"line_end":27,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"macro_rules! barstar {","highlight_start":1,"highlight_end":21}],"label":"when calling this macro","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `?`\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-2018.rs:48:15\n   |\nLL | macro_rules! barstar {\n   | -------------------- when calling this macro\n...\nLL |     barstar!(a?a); //~ ERROR no rules expected the token `?`\n   |               ^ no rules expected this token in macro call\n\n"}
[00:47:34] {"message":"aborting due to 12 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 12 previous errors\n\n"}
[00:47:34] ------------------------------------------
[00:47:34] 
[00:47:34] thread '[ui] ui/macros/macro-at-most-once-rep-2018.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:47:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:34] 
[00:47:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:47:34] 
[00:47:34] 
[00:47:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:34] 
[00:47:34] 
[00:47:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:34] Build completed unsuccessfully in 0:03:48
[00:47:34] Build completed unsuccessfully in 0:03:48
[00:47:34] Makefile:58: recipe for target 'check' failed
[00:47:34] make: *** [check] Error 1
ores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c61cdb7
travis_time:start:0c61cdb7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12bf83aa
$ dmesg | grep -i kill
