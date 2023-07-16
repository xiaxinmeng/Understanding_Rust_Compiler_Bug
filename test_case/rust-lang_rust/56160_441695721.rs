plain
travis_time:end:1526d206:start=1543244844698815642,finish=1543244846101994454,duration=1403178812
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:02] .................................................................................................... 100/5069
[00:51:05] .................................................................................................... 200/5069
[00:51:08] .............................ii............................................ii...................ii.. 300/5069
[00:51:11] ..............................................................................................iii... 400/5069
[00:51:14] .....iiiiiiii.iii............................iii............................................i....... 500/5069
[00:51:22] .................................................................................................... 700/5069
[00:51:27] ...................................................................................................i 800/5069
[00:51:31] ...........i........................................................................................ 900/5069
[00:51:35] ..................iiiii..................ii.iiii.................................................... 1000/5069
[00:51:35] ..................iiiii..................ii.iiii.................................................... 1000/5069
[00:51:37] .............................................................................................iiiiiii 1100/5069
[00:51:42] .................................................................................................... 1300/5069
[00:51:44] .................................................................................................... 1400/5069
[00:51:46] ................................................i................................................... 1500/5069
[00:51:50] .................i.........ii.........................................................i............. 1600/5069
---
[00:52:15] .................................................................................................... 2300/5069
[00:52:19] .................................................................................................... 2400/5069
[00:52:23] .................................................................................................... 2500/5069
[00:52:26] .................................................................................................... 2600/5069
[00:52:30] .....iiiiiiiii...................................................................................... 2700/5069
[00:52:36] .................................................................................................... 2900/5069
[00:52:40] .................................................................................................... 3000/5069
[00:52:43] ....................................................................i............................... 3100/5069
[00:52:46] .................................................................................................... 3200/5069
---
[00:53:54] .................................................................................................... 100/2886
[00:54:05] .................................................................................i.................. 200/2886
[00:54:13] .................................................................................................... 300/2886
[00:54:24] .................................................................................................... 400/2886
[00:54:33] ........................................F........................................................... 500/2886
[00:54:45] .................................F.................................................................. 600/2886
[00:55:11] .................................................................................................... 800/2886
[00:55:20] .................................................................................................... 900/2886
[00:55:36] .................................................................................................... 1000/2886
[00:55:49] .................................................................................................... 1100/2886
---
[01:00:10] failures:
[01:00:10] 
[01:00:10] ---- [run-pass] run-pass/consts/const-binops.rs stdout ----
[01:00:10] normalized stderr:
[01:00:10] warning: boolean short circuiting operators in constants do not actually short circuit. Thus new const eval features are not accessible in constants.
[01:00:10]    |
[01:00:10]    |
[01:00:10] LL | static N: bool = true && false;
[01:00:10]    |                       ^^ help: use a bit operator instead: `&`
[01:00:10] 
[01:00:10] warning: boolean short circuiting operators in constants do not actually short circuit. Thus ures are not accessible in constants.","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/consts/const-binops.rs","byte_start":1029,"byte_end":1031,"line_start":37,"line_end":37,"column_start":23,"column_end":25,"is_primary":true,"text":[{"text":"static N: bool = true && false;","highlight_start":23,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a bit operator instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/consts/const-binops.rs","byte_start":1029,"byte_end":1031,"line_start":37,"line_end":37,"column_start":23,"column_end":25,"is_primary":true,"text":[{"text":"static N: bool = true && false;","highlight_start":23,"highlight_end":25}],"label":null,"suggested_replacement":"&","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: boolean short circuiting operators in constants do not actually short circuit. Thus new const eval features are not accessible in constants.\n  --> /checkout/src/test/run-pass/consts/const-binops.rs:37:23\n   |\nLL | static N: bool = true && false;\n   |                       ^^ help: use a bit operator instead: `&`\n\n"}
[01:00:10] {"message":"boolean short circuiting operators in constants do not actually short circuit. Thus new const eval features are not accessible in constants.","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/consts/const-binops.rs","byte_start":1062,"byte_end":1064,"line_start":39,"line_end":39,"column_start":23,"column_end":25,"is_primary":true,"text":[{"text":"static O: bool = true || false;","highlight_start":23,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a bit operator instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/consts/const-binops.rs","byte_start":1062,"byte_end":1064,"line_start":39,"line_end":39,"column_start":23,"column_end":25,"is_primary":true,"text":[{"text":"static O: bool = true || false;","highlight_start":23,"highlight_end":25}],"label":null,"suggested_replacement":"|","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: boolean short circuiting operators in constants do not actually short circuit. Thus new const eval features are not accessible in constants.\n  --> /checkout/src/test/run-pass/consts/const-binops.rs:39:23\n   |\nLL | static O: bool = true || false;\n   |                       ^^ help: use a bit operator instead: `|`\n\n"}
[01:00:10] ------------------------------------------
[01:00:10] 
[01:00:10] thread '[run-pass] run-pass/consts/const-binops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:00:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:10] 
[01:00:10] ---- [run-pass] run-pass/ctfe/issue-37550.rs stdout ----
[01:00:10] 
[01:00:10] error: test compilation failed although it shouldn't!
[01:00:10] status: exit code: 1
[01:00:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/ctfe/istable (see issue #48821)\n  --> /checkout/src/test/run-pass/ctfe/issue-37550.rs:19:13\n   |\nLL |     let x = || t;\n   |             ^^^^\n   |\n   = help: add #![feature(const_let)] to the crate attributes to enable\n\n"}
[01:00:10] {"message":"statements in constant functions are unstable (see issue #48821)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n