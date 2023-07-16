\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-52891.rs","byte_start":916,"byte_end":930,"line_start":36,"line_end":36,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"use issue_52891::n; //~ ERROR `n` is defined multiple times","highlight_start":5,"highlight_end":19}],"label":"`n` reimported here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-52891.rs","byte_start":883,"byte_end":897,"line_start":34,"line_end":34,"column_start":5,"column_end":19,"is_primary":false,"text":[{"text":"use issue_52891::n;","highlight_start":5,"highlight_end":19}],"label":"previous import of the module `n` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`n` must be defined only once in the type namespace of this module","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove unnecessary import","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-52891.rs","byte_start":879,"byte_end":898,"line_start":34,"line_end":34,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"use issue_52891::n;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0252]: the name `n` is defined multiple times\n  --> /checkout/src/test/ui/issues/issue-52891.rs:36:5\n   |\nLL | use issue_52891::n;\n   | -------------------\n   | |   |\n   | |   previous import of the module `n` here\n   | help: remove unnecessary import\nLL | #[macro_use]\nLL | use issue_52891::n; //~ ERROR `n` is defined multiple times\n   |     ^^^^^^^^^^^^^^ `n` reimported here\n   |\n   = note: `n` must be defined only once in the type namespace of this module\n\n"}
[01:01:49] {"message":"aborting due to 10 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 10 previous errors\n\n"}
[01:01:49] {"message":"Some errors occurred: E0252, E0254.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0252, E0254.\n"}
[01:01:49] 
[01:01:49] ------------------------------------------
[01:01:49] 
[01:01:49] thread '[ui] ui/issues/issue-52891.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:01:49] thread '[ui] ui/issues/issue-52891.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:01:49] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:01:49] 
[01:01:49] ---- [ui] ui/issues/issue-57819.rs stdout ----
[01:01:49] diff of stderr:
[01:01:49] 
[01:01:49] 14   --> $DIR/issue-57819.rs:25:10
[01:01:49] 15    |
[01:01:49] 16 LL |     bar::<<<T as Foo>::Output>();
[01:01:49] -    |          ^ help: remove extra angle bracket
[01:01:49] +    |          ^ help: maybe remove extra angle bracket
[01:01:49] 19 error: unmatched angle brackets
[01:01:49] 20   --> $DIR/issue-57819.rs:34:48
[01:01:49] 
[01:01:49] 38   --> $DIR/issue-57819.rs:43:48
[01:01:49] 38   --> $DIR/issue-57819.rs:43:48
[01:01:49] 39    |
[01:01:49] 40 LL |     let _ = vec![1, 2, 3].into_iter().collect::<<Vec<usize>>();
[01:01:49] -    |                                                ^ help: remove extra angle bracket
[01:01:49] +    |                                                ^ help: maybe remove extra angle bracket
[01:01:49] 43 error: aborting due to 7 previous errors
[01:01:49] 44 
[01:01:49] 
[01:01:49] 
[01:01:49] 
[01:01:49] The actual stderr differed from the expected stderr.
[01:01:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57819/issue-57819.stderr
[01:01:49] To update references, rerun the tests and pass the `--bless` flag
[01:01:49] To only update this specific test, also pass `--test-args issues/issue-57819.rs`
[01:01:49] error: 1 errors occurred comparing output.
[01:01:49] status: exit code: 1
[01:01:49] status: exit code: 1
[01:01:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-57819.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57819/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57819/auxiliary" "-A" "unused"
[01:01:49] ------------------------------------------
[01:01:49] 
[01:01:49] ------------------------------------------
[01:01:49] stderr:
[01:01:49] stderr:
[01:01:49] ------------------------------------------
[01:01:49] {"message":"unmatched angle brackets","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":421,"byte_end":424,"line_start":19,"line_end":19,"column_start":10,"column_end":13,"is_primary":true,"text":[{"text":"    bar::<<<<<T as Foo>::Output>();","highlight_start":10,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove extra angle brackets","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":421,"byte_end":424,"line_start":19,"line_end":19,"column_start":10,"column_end":13,"is_primary":true,"text":[{"text":"    bar::<<<<<T as Foo>::Output>();","highlight_start":10,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unmatched angle brackets\n  --> /checkout/src/test/ui/issues/issue-57819.rs:19:10\n   |\nLL |     bar::<<<<<T as Foo>::Output>();\n   |          ^^^ help: remove extra angle brackets\n\n"}
[01:01:49] {"message":"unmatched angle brackets","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":497,"byte_end":499,"line_start":22,"line_end":22,"column_start":10,"column_end":12,"is_primary":true,"text":[{"text":"    bar::<<<<T as Foo>::Output>();","highlight_start":10,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove extra angle brackets","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":497,"byte_end":499,"line_start":22,"line_end":22,"column_start":10,"column_end":12,"is_primary":true,"text":[{"text":"    bar::<<<<T as Foo>::Output>();","highlight_start":10,"highlight_end":12}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unmatched angle brackets\n  --> /checkout/src/test/ui/issues/issue-57819.rs:22:10\n   |\nLL |     bar::<<<<T as Foo>::Output>();\n   |          ^^ help: remove extra angle brackets\n\n"}
[01:01:49] {"message":"unmatched angle bracket","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":572,"byte_end":573,"line_start":25,"line_end":25,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"    bar::<<<T as Foo>::Output>();","highlight_start":10,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"maybe remove extra angle bracket","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":572,"byte_end":573,"line_start":25,"line_end":25,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"    bar::<<<T as Foo>::Output>();","highlight_start":10,"highlight_end":11}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unmatched angle bracket\n  --> /checkout/src/test/ui/issues/issue-57819.rs:25:10\n   |\nLL |     bar::<<<T as Foo>::Output>();\n   |          ^ help: maybe remove extra angle bracket\n\n"}
[01:01:49] {"message":"unmatched angle brackets","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":748,"byte_end":752,"line_start":34,"line_end":34,"column_start":48,"column_end":52,"is_primary":true,"text":[{"text":"    let _ = vec![1, 2, 3].into_iter().collect::<<<<<Vec<usize>>();","highlight_start":48,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove extra angle brackets","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":748,"byte_end":752,"line_start":34,"line_end":34,"column_start":48,"column_end":52,"is_primary":true,"text":[{"text":"    let _ = vec![1, 2, 3].into_iter().collect::<<<<<Vec<usize>>();","highlight_start":48,"highlight_end":52}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unmatched angle brackets\n  --> /checkout/src/test/ui/issues/issue-57819.rs:34:48\n   |\nLL |     let _ = vec![1, 2, 3].into_iter().collect::<<<<<Vec<usize>>();\n   |                                                ^^^^ help: remove extra angle brackets\n\n"}
[01:01:49] {"message":"unmatched angle brackets","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":855,"byte_end":858,"line_start":37,"line_end":37,"column_start":48,"column_end":51,"is_primary":true,"text":[{"text":"    let _ = vec![1, 2, 3].into_iter().collect::<<<<Vec<usize>>();","highlight_start":48,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove extra angle brackets","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":855,"byte_end":858,"line_start":37,"line_end":37,"column_start":48,"column_end":51,"is_primary":true,"text":[{"text":"    let _ = vec![1, 2, 3].into_iter().collect::<<<<Vec<usize>>();","highlight_start":48,"highlight_end":51}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unmatched angle brackets\n  --> /checkout/src/test/ui/issues/issue-57819.rs:37:48\n   |\nLL |     let _ = vec![1, 2, 3].into_iter().collect::<<<<Vec<usize>>();\n   |                                                ^^^ help: remove extra angle brackets\n\n"}
[01:01:49] {"message":"unmatched angle brackets","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":961,"byte_end":963,"line_start":40,"line_end":40,"column_start":48,"column_end":50,"is_primary":true,"text":[{"text":"    let _ = vec![1, 2, 3].into_iter().collect::<<<Vec<usize>>();","highlight_start":48,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove extra angle brackets","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":961,"byte_end":963,"line_start":40,"line_end":40,"column_start":48,"column_end":50,"is_primary":true,"text":[{"text":"    let _ = vec![1, 2, 3].into_iter().collect::<<<Vec<usize>>();","highlight_start":48,"highlight_end":50}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unmatched angle brackets\n  --> /checkout/src/test/ui/issues/issue-57819.rs:40:48\n   |\nLL |     let _ = vec![1, 2, 3].into_iter().collect::<<<Vec<usize>>();\n   |                                                ^^ help: remove extra angle brackets\n\n"}
[01:01:49] {"message":"unmatched angle bracket","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":1066,"byte_end":1067,"line_start":43,"line_end":43,"column_start":48,"column_end":49,"is_primary":true,"text":[{"text":"    let _ = vec![1, 2, 3].into_iter().collect::<<Vec<usize>>();","highlight_start":48,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"maybe remove extra angle bracket","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57819.rs","byte_start":1066,"byte_end":1067,"line_start":43,"line_end":43,"column_start":48,"column_end":49,"is_primary":true,"text":[{"text":"    let _ = vec![1, 2, 3].into_iter().collect::<<Vec<usize>>();","highlight_start":48,"highlight_end":49}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unmatched angle bracket\n  --> /checkout/src/test/ui/issues/issue-57819.rs:43:48\n   |\nLL |     let _ = vec![1, 2, 3].into_iter().collect::<<Vec<usize>>();\n   |                                                ^ help: maybe remove extra angle bracket\n\n"}
[01:01:49] 
[01:01:49] ------------------------------------------
[01:01:49] 
[01:01:49] thread '[ui] ui/issues/issue-57819.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:01:49] 
[01:01:49] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:01:49] 
[01:01:49] 
[01:01:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:49] 
[01:01:49] 
[01:01:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:49] Build completed unsuccessfully in 0:04:06
[01:01:49] Build completed unsuccessfully in 0:04:06
[01:01:49] Makefile:48: recipe for target 'check' failed
[01:01:49] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cffc54c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 12:25:48 UTC 2019
Tue Feb  5 12:25:48 UTC 2019
bj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05f34864
travis_time:start:05f34864
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:174a3128
$ dmesg | grep -i kill
