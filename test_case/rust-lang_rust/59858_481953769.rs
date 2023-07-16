plain
travis_time:end:1b862198:start=1554950340660246403,finish=1554950341694578638,duration=1034332235
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:04:04] .................................................................................................... 3000/5530
[01:04:07] .................................................................................................... 3100/5530
[01:04:10] .................................................................................................... 3200/5530
[01:04:15] .................................................................................................... 3300/5530
[01:04:18] ..............................i...............F..................................................... 3400/5530
[01:04:25] ....ii...i..ii...................................................................................... 3600/5530
[01:04:29] .................................................................................................... 3700/5530
[01:04:33] .................................................................................................... 3800/5530
[01:04:36] ...............ii................................................................................... 3900/5530
---
[01:05:37] 
[01:05:37] ---- [ui] ui/macros/macro-multiple-matcher-bindings.rs stdout ----
[01:05:37] diff of stderr:
[01:05:37] 
[01:05:37] + warning: lint `duplicate_matcher_binding_name` has been removed: `converted into hard error, see https://github.com/rust-lang/rust/issues/57742`
[01:05:37] +    |
[01:05:37] +    |
[01:05:37] + LL | #![warn(duplicate_matcher_binding_name)]
[01:05:37] +    |
[01:05:37] +    = note: #[warn(renamed_and_removed_lints)] on by default
[01:05:37] + 
[01:05:37] 1 error: duplicate matcher binding
[01:05:37] 1 error: duplicate matcher binding
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:7:16
[01:05:37] +   --> $DIR/macro-multiple-matcher-bindings.rs:8:16
[01:05:37] 3    |
[01:05:37] 4 LL |     ($a:ident, $a:ident) => {};
[01:05:37] 
[01:05:37] 6    |
[01:05:37] 7 note: previous declaration was here
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:7:6
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:7:6
[01:05:37] +   --> $DIR/macro-multiple-matcher-bindings.rs:8:6
[01:05:37] 9    |
[01:05:37] 10 LL |     ($a:ident, $a:ident) => {};
[01:05:37] 
[01:05:37] 12 
[01:05:37] 13 error: duplicate matcher binding
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:8:16
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:8:16
[01:05:37] +   --> $DIR/macro-multiple-matcher-bindings.rs:9:16
[01:05:37] 15    |
[01:05:37] 16 LL |     ($a:ident, $a:path) => {};
[01:05:37] 
[01:05:37] 18    |
[01:05:37] 19 note: previous declaration was here
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:8:6
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:8:6
[01:05:37] +   --> $DIR/macro-multiple-matcher-bindings.rs:9:6
[01:05:37] 21    |
[01:05:37] 22 LL |     ($a:ident, $a:path) => {};
[01:05:37] 
[01:05:37] 24 
[01:05:37] 25 error: duplicate matcher binding
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:17:18
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:17:18
[01:05:37] +   --> $DIR/macro-multiple-matcher-bindings.rs:18:18
[01:05:37] 27    |
[01:05:37] 28 LL |     ($a:ident, $($a:ident),*) => {};
[01:05:37] 
[01:05:37] 30    |
[01:05:37] 31 note: previous declaration was here
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:17:6
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:17:6
[01:05:37] +   --> $DIR/macro-multiple-matcher-bindings.rs:18:6
[01:05:37] 33    |
[01:05:37] 34 LL |     ($a:ident, $($a:ident),*) => {};
[01:05:37] 
[01:05:37] 36 
[01:05:37] 37 error: duplicate matcher binding
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:18:25
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:18:25
[01:05:37] +   --> $DIR/macro-multiple-matcher-bindings.rs:19:25
[01:05:37] 39    |
[01:05:37] 40 LL |     ($($a:ident)+ # $($($a:path),+);*) => {};
[01:05:37] 
[01:05:37] 42    |
[01:05:37] 43 note: previous declaration was here
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:18:8
[01:05:37] -   --> $DIR/macro-multiple-matcher-bindings.rs:18:8
[01:05:37] +   --> $DIR/macro-multiple-matcher-bindings.rs:19:8
[01:05:37] 45    |
[01:05:37] 46 LL |     ($($a:ident)+ # $($($a:path),+);*) => {};
[01:05:37] 
[01:05:37] 
[01:05:37] The actual stderr differed from the expected stderr.
[01:05:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-multiple-matcher-bindings/macro-multiple-matcher-bindings.stderr
[01:05:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-multiple-matcher-bindings/macro-multiple-matcher-bindings.stderr
[01:05:37] To update references, rerun the tests and pass the `--bless` flag
[01:05:37] To only update this specific test, also pass `--test-args macros/macro-multiple-matcher-bindings.rs`
[01:05:37] error: 1 errors occurred comparing output.
[01:05:37] status: exit code: 1
[01:05:37] status: exit code: 1
[01:05:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-multiple-matcher-bindings/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-multiple-matcher-bindings/auxiliary" "-A" "unused"
[01:05:37] ------------------------------------------
[01:05:37] 
[01:05:37] ------------------------------------------
[01:05:37] stderr:
[01:05:37] stderr:
[01:05:37] ------------------------------------------
[01:05:37] {"message":"lint `duplicate_matcher_binding_name` has been removed: `converted into hard error, see https://github.com/rust-lang/rust/issues/57742`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs","byte_start":152,"byte_end":182,"line_start":5,"line_end":5,"column_start":9,"column_end":39,"is_primary":true,"text":[{"text":"#![warn(duplicate_matcher_binding_name)]","highlight_start":9,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(renamed_and_removed_lints)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: lint `duplicate_matcher_binding_name` has been removed: `converted into hard error, see https://github.com/rust-lang/rust/issues/57742`\n  --> /checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs:5:9\n   |\nLL | #![warn(duplicate_matcher_binding_name)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(renamed_and_removed_lints)] on by default\n\n"}
[01:05:37] {"message":"duplicate matcher binding","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs","byte_start":221,"byte_end":229,"line_start":8,"line_end":8,"column_start":16,"column_end":24,"is_primary":true,"text":[{"text":"    ($a:ident, $a:ident) => {}; //~ERROR duplicate matcher binding","highlight_start":16,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"previous declaration was here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs","byte_start":211,"byte_end":219,"line_start":8,"line_end":8,"column_start":6,"column_end":14,"is_primary":true,"text":[{"text":"    ($a:ident, $a:ident) => {}; //~ERROR duplicate matcher binding","highlight_start":6,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: duplicate matcher binding\n  --> /checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs:8:16\n   |\nLL |     ($a:ident, $a:ident) => {}; //~ERROR duplicate matcher binding\n   |                ^^^^^^^^\n   |\nnote: previous declaration was here\n  --> /checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs:8:6\n   |\nLL |     ($a:ident, $a:ident) => {}; //~ERROR duplicate matcher binding\n   |      ^^^^^^^^\n\n"}
[01:05:37] {"message":"duplicate matcher binding","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs","byte_start":288,"byte_end":295,"line_start":9,"line_end":9,"column_start":16,"column_end":23,"is_primary":true,"text":[{"text":"    ($a:ident, $a:path) => {};  //~ERROR duplicate matcher binding","highlight_start":16,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"previous declaration was here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs","byte_start":278,"byte_end":286,"line_start":9,"line_end":9,"column_start":6,"column_end":14,"is_primary":true,"text":[{"text":"    ($a:ident, $a:path) => {};  //~ERROR duplicate matcher binding","highlight_start":6,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: duplicate matcher binding\n  --> /checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs:9:16\n   |\nLL |     ($a:ident, $a:path) => {};  //~ERROR duplicate matcher binding\n   |                ^^^^^^^\n   |\nnote: previous declaration was here\n  --> /checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs:9:6\n   |\nLL |     ($a:ident, $a:path) => {};  //~ERROR duplicate matcher binding\n   |      ^^^^^^^^\n\n"}
[01:05:37] {"message":"duplicate matcher binding","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs","byte_start":459,"byte_end":467,"line_start":18,"line_end":18,"column_start":18,"column_end":26,"is_primary":true,"text":[{"text":"    ($a:ident, $($a:ident),*) => {}; //~ERROR duplicate matcher binding","highlight_start":18,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"previous declaration was here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs","byte_start":447,"byte_end":455,"line_start":18,"line_end":18,"column_start":6,"column_end":14,"is_primary":true,"text":[{"text":"    ($a:ident, $($a:ident),*) => {}; //~ERROR duplicate matcher binding","highlight_start":6,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: duplicate matcher binding\n  --> /checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs:18:18\n   |\nLL |     ($a:ident, $($a:ident),*) => {}; //~ERROR duplicate matcher binding\n   |                  ^^^^^^^^\n   |\nnote: previous declaration was here\n  --> /checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs:18:6\n   |\nLL |     ($a:ident, $($a:ident),*) => {}; //~ERROR duplicate matcher binding\n   |      ^^^^^^^^\n\n"}
[01:05:37] {"message":"duplicate matcher binding","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs","byte_start":538,"byte_end":545,"line_start":19,"line_end":19,"column_start":25,"column_end":32,"is_primary":true,"text":[{"text":"    ($($a:ident)+ # $($($a:path),+);*) => {}; //~ERROR duplicate matcher binding","highlight_start":25,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"previous declaration was here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs","byte_start":521,"byte_end":529,"line_start":19,"line_end":19,"column_start":8,"column_end":16,"is_primary":true,"text":[{"text":"    ($($a:ident)+ # $($($a:path),+);*) => {}; //~ERROR duplicate matcher binding","highlight_start":8,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: duplicate matcher binding\n  --> /checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs:19:25\n   |\nLL |     ($($a:ident)+ # $($($a:path),+);*) => {}; //~ERROR duplicate matcher binding\n   |                         ^^^^^^^\n   |\nnote: previous declaration was here\n  --> /checkout/src/test/ui/macros/macro-multiple-matcher-bindings.rs:19:8\n   |\nLL |     ($($a:ident)+ # $($($a:path),+);*) => {}; //~ERROR duplicate matcher binding\n   |        ^^^^^^^^\n\n"}
[01:05:37] 
[01:05:37] ------------------------------------------
[01:05:37] 
[01:05:37] thread '[ui] ui/macros/macro-multiple-matcher-bindings.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:05:37] 
[01:05:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:05:37] 
[01:05:37] 
[01:05:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:37] 
[01:05:37] 
[01:05:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:37] Build completed unsuccessfully in 0:04:16
[01:05:37] Build completed unsuccessfully in 0:04:16
[01:05:37] make: *** [check] Error 1
[01:05:37] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0403acf6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 11 03:44:51 UTC 2019
