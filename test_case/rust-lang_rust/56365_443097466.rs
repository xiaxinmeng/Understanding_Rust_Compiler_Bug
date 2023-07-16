plain
travis_time:end:01de1e88:start=1543553526219730636,finish=1543553528589920223,duration=2370189587
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:39] .................................................................................................... 2500/5065
[00:46:42] .................................................................................................... 2600/5065
[00:46:46] .................................................................................................... 2700/5065
[00:46:50] .................................................................................................... 2800/5065
[00:46:52] .....................................................F.............................................. 2900/5065
[00:46:59] .....................................................................i.............................. 3100/5065
[00:47:02] .................................................................................................... 3200/5065
[00:47:05] ................................ii..i..ii........................................................... 3300/5065
[00:47:09] .................................................................................................... 3400/5065
---
[00:47:22] .................................................................................................... 4000/5065
[00:47:24] .................................................................................................... 4100/5065
[00:47:27] .................................................................................................... 4200/5065
[00:47:31] ................................................i................................................... 4300/5065
[00:47:36] .................................................................................F.................. 4400/5065
[00:47:42] .................................................................................................... 4600/5065
[00:47:45] .............................i...................................................................... 4700/5065
[00:47:49] .................................................................................................... 4800/5065
[00:47:52] .................................................................................................... 4900/5065
[00:47:52] .................................................................................................... 4900/5065
[00:47:54] .................................................................................................... 5000/5065
"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0531]: cannot find unit struct/variant or constant `Self` in this scope\n  --> /checkout/src/test/ui/keyword/keyword-self-as-identifier.rs:12:9\n   |\nLL |     let Self = 22; //~ ERROR cannot find unit struct/variant or constant `Self` in this scope\n   |         ^^^^ not found in this scope\n\n"}
[00:47:56] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:56] {"message":"For more information about this error, try `rustc --explain E0531`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0531`.\n"}
[00:47:56] ------------------------------------------
[00:47:56] 
[00:47:56] thread '[ui] ui/keyword/keyword-self-as-identifier.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:47:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:56] 29 
[00:47:56] 
[00:47:56] 
[00:47:56] The actual stderr differed from the expected stderr.
[00:47:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword-2/self_type_keyword-2.stderr
[00:47:56] To update references, rerun the tests and pass the `--bless` flag
[00:47:56] To only update this specific test, also pass `--test-args self/self_type_keyword-2.rs`
[00:47:56] error: 1 errors occurred comparing output.
[00:47:56] status: exit code: 1
[00:47:56] status: exit code: 1
[00:47:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/self_type_keyword-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword-2/auxiliary" "-A" "unused"
[00:47:56] ------------------------------------------
[00:47:56] 
[00:47:56] ------------------------------------------
[00:47:56] stderr:
[00:47:56] stderr:
[00:47:56] ------------------------------------------
[00:47:56] {"message":"unresolved import `self::Self`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n