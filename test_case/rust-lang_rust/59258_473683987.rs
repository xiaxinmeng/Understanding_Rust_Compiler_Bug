plain
travis_time:end:23f0b1f6:start=1552837168138477449,finish=1552837242087123565,duration=73948646116
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:12:43] .................................................................................................... 4000/5473
[01:12:45] .......................................................i............................................ 4100/5473
[01:12:48] .................................................................................................... 4200/5473
[01:12:57] .................................................................................................... 4300/5473
[01:13:03] ............................F....................................................................... 4400/5473
[01:13:10] .................................................................................................... 4600/5473
[01:13:15] ...............i.................................................................................... 4700/5473
[01:13:20] .................................................................................................... 4800/5473
[01:13:23] .................................................................................................... 4900/5473
---
[01:13:43] diff of stderr:
[01:13:43] 
[01:13:43] 20   --> $DIR/resolve-error.rs:44:10
[01:13:43] 21    |
[01:13:43] 22 LL | #[derive(attr_proc_macra)]
[01:13:43] -    |          ^^^^^^^^^^^^^^^ help: try: `attr_proc_macro`
[01:13:43] 24 
[01:13:43] 24 
[01:13:43] 25 error: cannot find macro `FooWithLongNama!` in this scope
[01:13:43] 
[01:13:43] 
[01:13:43] The actual stderr differed from the expected stderr.
[01:13:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/resolve-error.stderr
[01:13:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/resolve-error.stderr
[01:13:43] To update references, rerun the tests and pass the `--bless` flag
[01:13:43] To only update this specific test, also pass `--test-args proc-macro/resolve-error.rs`
[01:13:43] error: 1 errors occurred comparing output.
[01:13:43] status: exit code: 1
[01:13:43] status: exit code: 1
[01:13:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/resolve-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/auxiliary" "-A" "unused"
[01:13:43] ------------------------------------------
[01:13:43] 
[01:13:43] ------------------------------------------
[01:13:43] stderr:
[01:13:43] stderr:
[01:13:43] ------------------------------------------
[01:13:43] {"message":"cannot find derive macro `FooWithLongNan` in this scope","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":468,"byte_end":482,"line_start":26,"line_end":26,"column_start":10,"column_end":24,"is_primary":true,"text":[{"text":"#[derive(FooWithLongNan)]","highlight_start":10,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":468,"byte_end":482,"line_start":26,"line_end":26,"column_start":10,"column_end":24,"is_primary":true,"text":[{"text":"#[derive(FooWithLongNan)]","highlight_start":10,"highlight_end":24}],"label":null,"suggested_replacement":"FooWithLongName","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot find derive macro `FooWithLongNan` in this scope\n  --> /checkout/src/test/ui/proc-macro/resolve-error.rs:26:10\n   |\nLL | #[derive(FooWithLongNan)]\n   |          ^^^^^^^^^^^^^^ help: try: `FooWithLongName`\n\n"}
[01:13:43] {"message":"cannot find derive macro `Dlone` in this scope","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":677,"byte_end":682,"line_start":36,"line_end":36,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"#[derive(Dlone)]","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":677,"byte_end":682,"line_start":36,"line_end":36,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"#[derive(Dlone)]","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":"Clone","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot find derive macro `Dlone` in this scope\n  --> /checkout/src/test/ui/proc-macro/resolve-error.rs:36:10\n   |\nLL | #[derive(Dlone)]\n   |          ^^^^^ help: try: `Clone`\n\n"}
[01:13:43] {"message":"cannot find derive macro `Dlona` in this scope","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":728,"byte_end":733,"line_start":40,"line_end":40,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"#[derive(Dlona)]","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":728,"byte_end":733,"line_start":40,"line_end":40,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"#[derive(Dlona)]","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":"Clona","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot find derive macro `Dlona` in this scope\n  --> /checkout/src/test/ui/proc-macro/resolve-error.rs:40:10\n   |\nLL | #[derive(Dlona)]\n   |          ^^^^^ help: try: `Clona`\n\n"}
[01:13:43] {"message":"cannot find derive macro `attr_proc_macra` in this scope","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":779,"byte_end":794,"line_start":44,"line_end":44,"column_start":10,"column_end":25,"is_primary":true,"text":[{"text":"#[derive(attr_proc_macra)]","highlight_start":10,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: cannot find derive macro `attr_proc_macra` in this scope\n  --> /checkout/src/test/ui/proc-macro/resolve-error.rs:44:10\n   |\nLL | #[derive(attr_proc_macra)]\n   |          ^^^^^^^^^^^^^^^\n\n"}
[01:13:43] {"message":"cannot find macro `FooWithLongNama!` in this scope","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":847,"byte_end":862,"line_start":49,"line_end":49,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    FooWithLongNama!();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you could try the macro","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":847,"byte_end":862,"line_start":49,"line_end":49,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    FooWithLongNama!();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"FooWithLongNam","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot find macro `FooWithLongNama!` in this scope\n  --> /checkout/src/test/ui/proc-macro/resolve-error.rs:49:5\n   |\nLL |     FooWithLongNama!();\n   |     ^^^^^^^^^^^^^^^ help: you could try the macro: `FooWithLongNam`\n\n"}
[01:13:43] {"message":"cannot find macro `attr_proc_macra!` in this scope","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":899,"byte_end":914,"line_start":52,"line_end":52,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    attr_proc_macra!();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you could try the macro","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":899,"byte_end":914,"line_start":52,"line_end":52,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    attr_proc_macra!();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"attr_proc_mac","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot find macro `attr_proc_macra!` in this scope\n  --> /checkout/src/test/ui/proc-macro/resolve-error.rs:52:5\n   |\nLL |     attr_proc_macra!();\n   |     ^^^^^^^^^^^^^^^ help: you could try the macro: `attr_proc_mac`\n\n"}
[01:13:43] {"message":"cannot find macro `Dlona!` in this scope","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":951,"byte_end":956,"line_start":55,"line_end":55,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    Dlona!();","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: cannot find macro `Dlona!` in this scope\n  --> /checkout/src/test/ui/proc-macro/resolve-error.rs:55:5\n   |\nLL |     Dlona!();\n   |     ^^^^^\n\n"}
[01:13:43] {"message":"cannot find macro `bang_proc_macrp!` in this scope","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":993,"byte_end":1008,"line_start":58,"line_end":58,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    bang_proc_macrp!();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you could try the macro","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/resolve-error.rs","byte_start":993,"byte_end":1008,"line_start":58,"line_end":58,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    bang_proc_macrp!();","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"bang_proc_macro","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot find macro `bang_proc_macrp!` in this scope\n  --> /checkout/src/test/ui/proc-macro/resolve-error.rs:58:5\n   |\nLL |     bang_proc_macrp!();\n   |     ^^^^^^^^^^^^^^^ help: you could try the macro: `bang_proc_macro`\n\n"}
[01:13:43] 
[01:13:43] ------------------------------------------
[01:13:43] 
[01:13:43] thread '[ui] ui/proc-macro/resolve-error.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:13:43] 
[01:13:43] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:13:43] 
[01:13:43] 
[01:13:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:43] 
[01:13:43] 
[01:13:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:43] Build completed unsuccessfully in 0:04:23
[01:13:43] Build completed unsuccessfully in 0:04:23
[01:13:43] Makefile:48: recipe for target 'check' failed
[01:13:43] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:197bc96e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 17 16:54:33 UTC 2019
---
travis_time:end:1153e9e5:start=1552841675167289192,finish=1552841675174727259,duration=7438067
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0307859d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e1edcec
$ dmesg | grep -i kill
