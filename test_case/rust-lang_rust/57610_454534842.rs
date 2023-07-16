plain
travis_time:end:00c2ddf3:start=1547579361463827115,finish=1547579365026042169,duration=3562215054
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:03:01] .................................................................................................... 2500/5306
[01:03:04] .................................................................................................... 2600/5306
[01:03:08] .................................................................................................... 2700/5306
[01:03:13] .................................................................................................... 2800/5306
[01:03:16] ...........F........................................................................................ 2900/5306
[01:03:23] .................................................................................................... 3100/5306
[01:03:27] ..........................................................................................i......... 3200/5306
[01:03:30] .................................................................................................... 3300/5306
[01:03:33] ....................................................ii...i...ii..................................... 3400/5306
---
[01:04:45] 
[01:04:45] ---- [ui] ui/issues/issue-5067.rs stdout ----
[01:04:45] diff of stderr:
[01:04:45] 
[01:04:45] 1 error: repetition matches empty token tree
[01:04:45] +   --> $DIR/issue-5067.rs:9:8
[01:04:45] 3    |
[01:04:45] 3    |
[01:04:45] 4 LL |     ( $()* ) => {};
[01:04:45] 
[01:04:45] 6 
[01:04:45] 6 
[01:04:45] 7 error: repetition matches empty token tree
[01:04:45] +   --> $DIR/issue-5067.rs:11:8
[01:04:45] 9    |
[01:04:45] 9    |
[01:04:45] 10 LL |     ( $()+ ) => {};
[01:04:45] 
[01:04:45] 12 
[01:04:45] 12 
[01:04:45] 13 error: repetition matches empty token tree
[01:04:45] +   --> $DIR/issue-5067.rs:13:8
[01:04:45] 15    |
[01:04:45] 15    |
[01:04:45] + LL |     ( $()? ) => {};
[01:04:45] + 
[01:04:45] + 
[01:04:45] + error: repetition matches empty token tree
[01:04:45] +    |
[01:04:45] +    |
[01:04:45] 16 LL |     ( [$()*] ) => {};
[01:04:45] 18 
[01:04:45] 
[01:04:45] 
[01:04:45] 19 error: repetition matches empty token tree
[01:04:45] +   --> $DIR/issue-5067.rs:20:9
[01:04:45] 21    |
[01:04:45] 21    |
[01:04:45] 22 LL |     ( [$()+] ) => {};
[01:04:45] 
[01:04:45] 24 
[01:04:45] 24 
[01:04:45] 25 error: repetition matches empty token tree
[01:04:45] +   --> $DIR/issue-5067.rs:22:9
[01:04:45] 27    |
[01:04:45] 27    |
[01:04:45] + LL |     ( [$()?] ) => {};
[01:04:45] + 
[01:04:45] + 
[01:04:45] + error: repetition matches empty token tree
[01:04:45] +    |
[01:04:45] +    |
[01:04:45] 28 LL |     ( $($()* $(),* $(a)* $(a),* )* ) => {};
[01:04:45] 30 
[01:04:45] 
[01:04:45] 
[01:04:45] 31 error: repetition matches empty token tree
[01:04:45] +   --> $DIR/issue-5067.rs:29:8
[01:04:45] 33    |
[01:04:45] 33    |
[01:04:45] 34 LL |     ( $($()* $(),* $(a)* $(a),* )+ ) => {};
[01:04:45] 
[01:04:45] 36 
[01:04:45] 36 
[01:04:45] 37 error: repetition matches empty token tree
[01:04:45] +   --> $DIR/issue-5067.rs:31:8
[01:04:45] 39    |
[01:04:45] 39    |
[01:04:45] + LL |     ( $($()* $(),* $(a)* $(a),* )? ) => {};
[01:04:45] + 
[01:04:45] + 
[01:04:45] + error: repetition matches empty token tree
[01:04:45] +    |
[01:04:45] +    |
[01:04:45] + LL |     ( $($()? $(),* $(a)? $(a),* )* ) => {};
[01:04:45] + 
[01:04:45] + 
[01:04:45] + error: repetition matches empty token tree
[01:04:45] +    |
[01:04:45] +    |
[01:04:45] + LL |     ( $($()? $(),* $(a)? $(a),* )+ ) => {};
[01:04:45] + 
[01:04:45] + 
[01:04:45] + error: repetition matches empty token tree
[01:04:45] +    |
[01:04:45] +    |
[01:04:45] + LL |     ( $($()? $(),* $(a)? $(a),* )? ) => {};
[01:04:45] + 
[01:04:45] + 
[01:04:45] + error: repetition matches empty token tree
[01:04:45] +    |
[01:04:45] +    |
[01:04:45] 40 LL |     ( $(a $()+)* ) => {};
[01:04:45] 42 
[01:04:45] 
[01:04:45] 
[01:04:45] 43 error: repetition matches empty token tree
[01:04:45] +   --> $DIR/issue-5067.rs:49:12
[01:04:45] 45    |
[01:04:45] 45    |
[01:04:45] 46 LL |     ( $(a $()*)+ ) => {};
[01:04:45] 
[01:04:45] 48 
[01:04:45] 48 
[01:04:45] 49 error: repetition matches empty token tree
[01:04:45] +   --> $DIR/issue-5067.rs:51:12
[01:04:45] 51    |
[01:04:45] 51    |
[01:04:45] + LL |     ( $(a $()+)? ) => {};
[01:04:45] + 
[01:04:45] + 
[01:04:45] + error: repetition matches empty token tree
[01:04:45] +    |
[01:04:45] +    |
[01:04:45] + LL |     ( $(a $()?)+ ) => {};
[01:04:45] + 
[01:04:45] + 
[01:04:45] + error: repetition matches empty token tree
[01:04:45] +    |
[01:04:45] +    |
[01:04:45] 52 LL |     (a $e1:expr $($(, a $e2:expr)*)*) => ([$e1 $($(, $e2)*)*]);
[01:04:45] 54 
[01:04:45] 
[01:04:45] 
[01:04:45] 55 error: repetition matches empty token tree
[01:04:45] +   --> $DIR/issue-5067.rs:71:8
[01:04:45] 57    |
[01:04:45] 57    |
[01:04:45] - LL |     ( $()* ) => {}
[01:04:45] + LL |     ( $()* ) => {};
[01:04:45] 60 
[01:04:45] 60 
[01:04:45] - error: aborting due to 1{};","highlight_start":8,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:9:8\n   |\nLL |     ( $()* ) => {};\n   |        ^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":245,"byte_end":247,"line_start":11,"line_end":11,"column_start":8,"column_end":10,"is_primary":true,"text":[{"text":"    ( $()+ ) => {};","highlight_start":8,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:11:8\n   |\nLL |     ( $()+ ) => {};\n   |        ^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":316,"byte_end":318,"line_start":13,"line_end":13,"column_start":8,"column_end":10,"is_primary":true,"text":[{"text":"    ( $()? ) => {};","highlight_start":8,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:13:8\n   |\nLL |     ( $()? ) => {};\n   |        ^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":484,"byte_end":486,"line_start":18,"line_end":18,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    ( [$()*] ) => {};","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:18:9\n   |\nLL |     ( [$()*] ) => {};\n   |         ^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":557,"byte_end":559,"line_start":20,"line_end":20,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    ( [$()+] ) => {};","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:20:9\n   |\nLL |     ( [$()+] ) => {};\n   |         ^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":630,"byte_end":632,"line_start":22,"line_end":22,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    ( [$()?] ) => {};","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:22:9\n   |\nLL |     ( [$()?] ) => {};\n   |         ^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":802,"byte_end":828,"line_start":27,"line_end":27,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"    ( $($()* $(),* $(a)* $(a),* )* ) => {};","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:27:8\n   |\nLL |     ( $($()* $(),* $(a)* $(a),* )* ) => {};\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":897,"byte_end":923,"line_start":29,"line_end":29,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"    ( $($()* $(),* $(a)* $(a),* )+ ) => {};","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:29:8\n   |\nLL |     ( $($()* $(),* $(a)* $(a),* )+ ) => {};\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":992,"byte_end":1018,"line_start":31,"line_end":31,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"    ( $($()* $(),* $(a)* $(a),* )? ) => {};","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:31:8\n   |\nLL |     ( $($()* $(),* $(a)* $(a),* )? ) => {};\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":1087,"byte_end":1113,"line_start":33,"line_end":33,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"    ( $($()? $(),* $(a)? $(a),* )* ) => {};","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:33:8\n   |\nLL |     ( $($()? $(),* $(a)? $(a),* )* ) => {};\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":1182,"byte_end":1208,"line_start":35,"line_end":35,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"    ( $($()? $(),* $(a)? $(a),* )+ ) => {};","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:35:8\n   |\nLL |     ( $($()? $(),* $(a)? $(a),* )+ ) => {};\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":1277,"byte_end":1303,"line_start":37,"line_end":37,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"    ( $($()? $(),* $(a)? $(a),* )? ) => {};","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:37:8\n   |\nLL |     ( $($()? $(),* $(a)? $(a),* )? ) => {};\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":1696,"byte_end":1698,"line_start":47,"line_end":47,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"    ( $(a $()+)* ) => {};","highlight_start":12,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:47:12\n   |\nLL |     ( $(a $()+)* ) => {};\n   |            ^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":1773,"byte_end":1775,"line_start":49,"line_end":49,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"    ( $(a $()*)+ ) => {};","highlight_start":12,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:49:12\n   |\nLL |     ( $(a $()*)+ ) => {};\n   |            ^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":1850,"byte_end":1852,"line_start":51,"line_end":51,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"    ( $(a $()+)? ) => {};","highlight_start":12,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:51:12\n   |\nLL |     ( $(a $()+)? ) => {};\n   |            ^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":1927,"byte_end":1929,"line_start":53,"line_end":53,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"    ( $(a $()?)+ ) => {};","highlight_start":12,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:53:12\n   |\nLL |     ( $(a $()?)+ ) => {};\n   |            ^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":2067,"byte_end":2085,"line_start":60,"line_end":60,"column_start":18,"column_end":36,"is_primary":true,"text":[{"text":"    (a $e1:expr $($(, a $e2:expr)*)*) => ([$e1 $($(, $e2)*)*]);","highlight_start":18,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:60:18\n   |\nLL |     (a $e1:expr $($(, a $e2:expr)*)*) => ([$e1 $($(, $e2)*)*]);\n   |                  ^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:45] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-5067.rs","byte_start":2275,"byte_end":2277,"line_start":71,"line_end":71,"column_start":8,"column_end":10,"is_primary":true,"text":[{"text":"    ( $()* ) => {};","highlight_start":8,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-5067.rs:71:8\n   |\nLL |     ( $()* ) => {};\n   |        ^^\n\n"}
[01:04:45] {"message":"aborting due to 18 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 18 previous errors\n\n"}
[01:04:45] ------------------------------------------
[01:04:45] 
[01:04:45] thread '[ui] ui/issues/issue-5067.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:04:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:04:45] 
[01:04:45] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:04:45] 
[01:04:45] 
[01:04:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:45] 
[01:04:45] 
[01:04:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:45] Build completed unsuccessfully in 0:04:09
[01:04:45] Build completed unsuccessfully in 0:04:09
[01:04:45] make: *** [check] Error 1
[01:04:45] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f4d6d45
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 15 20:14:20 UTC 2019
---
travis_time:end:22066d4c:start=1547583262011044114,finish=1547583262016416477,duration=5372363
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00666b1c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'
