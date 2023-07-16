plain
travis_time:end:058361b1:start=1543182799014667170,finish=1543182853764650520,duration=54749983350
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:41] .................................................................................................... 200/5058
[00:46:44] .................................................................................................... 300/5058
[00:46:47] .................................................................................................... 400/5058
[00:46:50] .................................................................................................... 500/5058
[00:46:54] .............................i......F............................................................... 600/5058
[00:47:03] .......................................................................................i...........i 800/5058
[00:47:06] .................................................................................................... 900/5058
[00:47:10] ......iiiii......................................................................................... 1000/5058
[00:47:12] .................................................................................................... 1100/5058
---
[00:49:01] .................................................................................................... 4600/5058
[00:49:05] ......................i............................................................................. 4700/5058
[00:49:08] .................................................................................................... 4800/5058
[00:49:11] .................................................................................................... 4900/5058
32,"is_primary":true,"text":[{"text":"    mem::replace(&mut x, &mut y);","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":"y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:58:26\n   |\nLL |     mem::replace(&mut x, &mut y);\n   |                          ^^^^^^\n   |                          |\n   |                          expected enum `std::option::Option`, found mutable reference\n   |                          help: consider removing the borrow: `y`\n   |\n   = note: expected type `std::option::Option<S>`\n              found type `&mut std::option::Option<S>`\n\n"}
[00:49:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:15] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[00:49:15] ------------------------------------------
[00:49:15] 
[00:49:15] thread '[ui] ui/coercion/coerce-issue-49593-box-never.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:49:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:15] 
[00:49:15] 
[00:49:15] failures:
[00:49:15]     [ui] ui/coercion/coerce-issue-49593-box-never.rs
[00:49:15] 
[00:49:15] test result: FAILED. 5033 passed; 1 failed; 24 ignoredor target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e4296a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 25 22:43:37 UTC 2018
---
travis_time:end:0109a2fb:start=1543185818701633785,finish=1543185818707235483,duration=5601698
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0826cc3e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ 
