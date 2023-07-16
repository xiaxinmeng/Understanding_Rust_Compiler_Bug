plain
travis_time:end:0685b244:start=1541034754843401901,finish=1541034755959917994,duration=1116516093
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:end:00a16db4:start=1541034765536428720,finish=1541034765545123924,duration=8695204
travis_fold:end:before_script.2
travis_time:start:0b3783a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov  1 01:12:45 UTC 2018
Thu, 01 Nov 2018 01:12:45 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_time:start:30846e01
---
[00:47:38] .................................................................................................... 100/4983
[00:47:41] .................................................................................................... 200/4983
[00:47:44] ...........................................................................................ii....... 300/4983
[00:47:47] ..........................................................................................iii....... 400/4983
[00:47:49] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4983
[00:47:56] .................................................................................................... 700/4983
[00:48:02] ..................................................................i...........i..................... 800/4983
[00:48:05] ....................................................................................iiiii........... 900/4983
[00:48:08] .................................................................................................... 1000/4983
---
[00:48:43] .................................................................................................... 2200/4983
[00:48:47] .................................................................................................... 2300/4983
[00:48:50] .................................................................................................... 2400/4983
[00:48:54] .................................................................................................... 2500/4983
[00:48:57] ...................................................................iiiiiiiii........................ 2600/4983
[00:49:04] ..................ii................................................................................ 2800/4983
[00:49:06] .................................................................................................... 2900/4983
[00:49:10] .................................................................................................... 3000/4983
[00:49:13] .............i...................................................................................... 3100/4983
---
[00:52:36] .................................................................................................... 1500/2873
[00:52:47] ..........................................i......................................................... 1600/2873
[00:53:02] .................................................................................................... 1700/2873
[00:53:12] .................................................................................................... 1800/2873
[00:53:21] .............................................................F.FFF..i............................... 1900/2873
[00:53:53] .................................................................................................... 2100/2873
[00:53:59] .................................................................................................... 2200/2873
[00:54:15] ii.....................................................................i....i....................... 2300/2873
[00:54:28] ..............i..................................................................................... 2400/2873
[00:54:28] ..............i..................................................................................... 2400/2873
[00:54:41] .................................................................................................... 2500/2873
[00:55:04] .................................................................................................... 2600/2873
[00:55:13] .................................................................................................... 2700/2873
[00:55:22] .................................................................................................... 2800/2873
tion failed although it shouldn't!
[00:55:31] status: exit code: 1
[00:55:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/modules/mod_dir_path2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/modules/mod_dir_path2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/modules/mod_dir_path2/auxiliary"
[00:55:31] ------------------------------------------
[00:55:31] 
[00:55:31] ------------------------------------------
[00:55:31] stderr:
[00:55:31] stderr:
[00:55:31] ------------------------------------------
[00:55:31] {"message":"couldn't read /checkout/src/test/run-pass/modules/test.rs: No such file or directory (os error 2)","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/modules/mod_dir_path2.rs","byte_start":593,"byte_end":598,"line_start":17,"line_end":17,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"    pub mod syrup;","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: couldn't read /checkout/src/test/run-pass/modules/test.rs: No such file or directory (os error 2)\n  --> /checkout/src/test/run-pass/modules/mod_dir_path2.rs:17:13\n   |\nLL |     pub mod syrup;\n   |             ^^^^^\n\n"}
[00:55:31] {"message":"aborting due to previous erro file corresponding to the module exists. If you\nwant to use a module named `file_that_doesnt_exist`, you need to have a file\nnamed `file_that_doesnt_exist.rs` or `file_that_doesnt_exist/mod.rs` in the\nsame directory.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/modules/mod_dir_path_multi.rs","byte_start":569,"byte_end":573,"line_start":16,"line_end":16,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    pub mod test;","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"name the file either biscuits/test.rs or biscuits/test/mod.rs inside the directory \"/checkout/src/test/run-pass/modules\"","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0583]: file not found for module `test`\n  --> /checkout/src/test/run-pass/modules/mod_dir_path_multi.rs:16:13\n   |\nLL |     pub mod test;\n   |             ^^^^\n   |\n   = help: name the file either biscuits/test.rs or biscuits/test/mod.rs inside the directory \"/checkout/src/test/run-pass/modules\"\n\n"}
[00:55:31] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:55:31] {"message":"For more information about this error, try `rustc --explain E0583`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0583`.\n"}
[00:55:31] ------------------------------------------
[00:55:31] 
[00:55:31] thread '[run-pass] run-pass/modules/mod_dir_path_multi.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:55:31] 
[00:55:31] 
[00:55:31] ---- [run-pass] run-pass/modules/mod_dir_path3.rs stdout ----
[00:55:31] 
[00:55:31] error: test compilation failed although it shouldn't!
[00:55:31] status: exit code: 1
[00:55:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/modules/mod_dir_path3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/modules/mod_dir_path3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/modules/mod_dir_path3/auxiliary"
[00:55:31] ------------------------------------------
[00:55:31] 
[00:55:31] ------------------------------------------
[00:55:31] stderr:
[00:55:31] stderr:
[00:55:31] ------------------------------------------
[00:55:31] {"message":"file not found for module `test`","code":{"code":"E0583","explanation":"\nA file wasn't found for an out-of-line module.\n\nErroneous code example:\n\n