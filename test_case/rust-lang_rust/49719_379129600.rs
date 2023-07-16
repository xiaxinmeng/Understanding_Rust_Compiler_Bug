plain
Resolving deltas: 100% (612410/612410), completed with 4868 local objects.
---
[00:00:44] configure: rust.quiet-tests     := True
---
[00:39:37] ..........................................................................i.........................
[00:39:43] .................i..................................................................................
---
[00:40:18] ..................................................F...........................................i.....
[00:40:25] ....................................................................i...............................
---
[00:40:46] 1 error: `?` macro repetition does not allow a separator
[00:40:46] -   --> $DIR/macro-at-most-once-rep-ambig.rs:29:10
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:22:10
[00:40:46] 3    |
[00:40:46] 4 LL |     ($(a),?) => {} //~ ERROR `?` macro repetition does not allow a separator
[00:40:46] 5    |          ^
[00:40:46]
[00:40:46] 6
[00:40:46] 7 error: no rules expected the token `?`
[00:40:46] -   --> $DIR/macro-at-most-once-rep-ambig.rs:43:11
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:36:11
[00:40:46] 9    |
[00:40:46] 10 LL |     foo!(a?a?a); //~ ERROR no rules expected the token `?`
[00:40:46] 11    |           ^
[00:40:46]
[00:40:46] 12
[00:40:46] 13 error: no rules expected the token `?`
[00:40:46] -   --> $DIR/macro-at-most-once-rep-ambig.rs:44:11
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:37:11
[00:40:46] 15    |
[00:40:46] 16 LL |     foo!(a?a); //~ ERROR no rules expected the token `?`
[00:40:46] 17    |           ^
[00:40:46]
[00:40:46] 18
[00:40:46] 19 error: no rules expected the token `?`
[00:40:46] -   --> $DIR/macro-at-most-once-rep-ambig.rs:45:11
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:38:11
[00:40:46] 21    |
[00:40:46] 22 LL |     foo!(a?); //~ ERROR no rules expected the token `?`
[00:40:46] 23    |           ^
[00:40:46]
[00:40:46] 24
[00:40:46] 25 error: unexpected end of macro invocation
[00:40:46] -   --> $DIR/macro-at-most-once-rep-ambig.rs:46:5
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:39:5
[00:40:46] 27    |
[00:40:46] 28 LL |     barplus!(); //~ ERROR unexpected end of macro invocation
[00:40:46] 29    |     ^^^^^^^^^^^
[00:40:46]
[00:40:46] 30
[00:40:46] 31 error: unexpected end of macro invocation
[00:40:46] -   --> $DIR/macro-at-most-once-rep-ambig.rs:47:5
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:40:5
[00:40:46] 33    |
[00:40:46] 34 LL |     barstar!(); //~ ERROR unexpected end of macro invocation
[00:40:46] 35    |     ^^^^^^^^^^^
[00:40:46]
[00:40:46] 36
[00:40:46] 37 error: no rules expected the token `?`
[00:40:46] -   --> $DIR/macro-at-most-once-rep-ambig.rs:48:15
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:41:15
[00:40:46] 39    |
[00:40:46] 40 LL |     barplus!(a?); //~ ERROR no rules expected the token `?`
[00:40:46] 41    |               ^
[00:40:46]
[00:40:46] 42
[00:40:46] 43 error: unexpected end of macro invocation
[00:40:46] -   --> $DIR/macro-at-most-once-rep-ambig.rs:49:14
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:42:14
[00:40:46] 45    |
[00:40:46] 46 LL |     barplus!(a); //~ ERROR unexpected end of macro invocation
[00:40:46] 47    |              ^
[00:40:46]
[00:40:46] 48
[00:40:46] 49 error: no rules expected the token `?`
[00:40:46] -   --> $DIR/macro-at-most-once-rep-ambig.rs:50:15
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:43:15
[00:40:46] 51    |
[00:40:46] 52 LL |     barstar!(a?); //~ ERROR no rules expected the token `?`
[00:40:46] 53    |               ^
[00:40:46]
[00:40:46] 54
[00:40:46] 55 error: unexpected end of macro invocation
[00:40:46] -   --> $DIR/macro-at-most-once-rep-ambig.rs:51:14
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:44:14
[00:40:46] 57    |
[00:40:46] 58 LL |     barstar!(a); //~ ERROR unexpected end of macro invocation
[00:40:46] 59    |              ^
[00:40:46]
[00:40:46] 60
[00:40:46] - error: aborting due to 10 previous errors
[00:40:46] + error: unexpected end of macro invocation
[00:40:46] +   --> $DIR/macro-at-most-once-rep-ambig.rs:47:14
[00:40:46] +    |
[00:40:46] + LL |     barplus!(a); // ok
[00:40:46] +    |              ^
---
[00:40:46] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'macros/macro-at-most-once-rep-ambig.rs'
[00:40:46]
[00:40:46] error: 1 errors occurred comparing output.
[00:40:46] status: exit code: 101
[00:40:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-at-most-once-rep-ambig.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-at-most-once-rep-ambig.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:40:46] {"message":"`?` macro repetition does not allow a separator","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs","byte_start":780,"byte_end":781,"line_start":22,"line_end":22,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"    ($(a),?) => {} //~ ERROR `?` macro repetition does not allow a separator","highlight_start":10,"highlight_end":11}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: `?` macro repetition does not allow a separator\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs:22:10\n   |\nLL |     ($(a),?) => {} //~ ERROR `?` macro repetition does not allow a separator\n   |          ^\n\n"}
[00:40:46] {"message":"no rules expected the token `?`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs","byte_start":1025,"byte_end":1026,"line_start":36,"line_end":36,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    foo!(a?a?a); //~ ERROR no rules expected the token `?`","highlight_start":11,"highlight_end":12}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `?`\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs:36:11\n   |\nLL |     foo!(a?a?a); //~ ERROR no rules expected the token `?`\n   |           ^\n\n"}
[00:40:46] {"message":"no rules expected the token `?`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs","byte_start":1084,"byte_end":1085,"line_start":37,"line_end":37,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    foo!(a?a); //~ ERROR no rules expected the token `?`","highlight_start":11,"highlight_end":12}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `?`\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs:37:11\n   |\nLL |     foo!(a?a); //~ ERROR no rules expected the token `?`\n   |           ^\n\n"}
[00:40:46] {"message":"no rules expected the token `?`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs","byte_start":1141,"byte_end":1142,"line_start":38,"line_end":38,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    foo!(a?); //~ ERROR no rules expected the token `?`","highlight_start":11,"highlight_end":12}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `?`\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs:38:11\n   |\nLL |     foo!(a?); //~ ERROR no rules expected the token `?`\n   |           ^\n\n"}
[00:40:46] {"message":"unexpected end of macro invocation","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs","byte_start":1191,"byte_end":1202,"line_start":39,"line_end":39,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    barplus!(); //~ ERROR unexpected end of macro invocation","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs:39:5\n   |\nLL |     barplus!(); //~ ERROR unexpected end of macro invocation\n   |     ^^^^^^^^^^^\n\n"}
[00:40:46] {"message":"unexpected end of macro invocation","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs","byte_start":1252,"byte_end":1263,"line_start":40,"line_end":40,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    barstar!(); //~ ERROR unexpected end of macro invocation","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs:40:5\n   |\nLL |     barstar!(); //~ ERROR unexpected end of macro invocation\n   |     ^^^^^^^^^^^\n\n"}
[00:40:46] {"message":"no rules expected the token `?`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs","byte_start":1323,"byte_end":1324,"line_start":41,"line_end":41,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"    barplus!(a?); //~ ERROR no rules expected the token `?`","highlight_start":15,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `?`\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs:41:15\n   |\nLL |     barplus!(a?); //~ ERROR no rules expected the token `?`\n   |               ^\n\n"}
[00:40:46] {"message":"unexpected end of macro invocation","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs","byte_start":1382,"byte_end":1383,"line_start":42,"line_end":42,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"    barplus!(a); //~ ERROR unexpected end of macro invocation","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs:42:14\n   |\nLL |     barplus!(a); //~ ERROR unexpected end of macro invocation\n   |              ^\n\n"}
[00:40:46] {"message":"no rules expected the token `?`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-at-most-once-rep-ted_replacement":null,"expansion":null}],"children":[],"rendered":"error: unexpected end of macro invocation\n  --> /checkout/src/test/ui/macros/macro-at-most-once-rep-ambig.rs:47:14\n   |\nLL |     barplus!(a); // ok\n   |              ^\n\n"}
[00:40:46] {"message":"aborting due to 11 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 11 previous errors\n\n"}
---
[00:40:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:40:46] expected success, got: exit code: 101
[00:40:46]
[00:40:46]
[00:40:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:40:46] Build completed unsuccessfully in 0:02:18
[00:40:46] Makefile:58: recipe for target 'check' failed
[00:40:46] make: *** [check] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:084a03fa:start=1522980977446315829,finish=1522980977453955318,duration=7639489
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0f49c4f8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0f49c4f8:start=1522980977460915259,finish=1522980977468588665,duration=7673406
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19721a36
$ dmesg | grep -i kill
[   10.700008] init: failsafe main process (1093) killed by TERM signal
