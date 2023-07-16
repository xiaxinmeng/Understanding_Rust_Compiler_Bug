plain
travis_time:end:08f93334:start=1544456192608146835,finish=1544456193698296910,duration=1090150075
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:58] .................................................................................i.................. 200/2926
[00:50:06] .................................................................................................... 300/2926
[00:50:16] .................................................................................................... 400/2926
[00:50:25] .................................................................................................... 500/2926
[00:50:36] ............................................F....................................................... 600/2926
[00:51:01] .................................................................................................... 800/2926
[00:51:10] .................................................................................................... 900/2926
[00:51:25] .................................................................................................... 1000/2926
[00:51:37] .................................................................................................... 1100/2926
---
[00:55:11] .................................................................................................... 2600/2926
[00:55:19] .................................................................................................... 2700/2926
[00:55:29] .................................................................................................... 2800/2926
[00:55:41] .................................................................................................... 2900/2926
pers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/ctfe/references/auxiliary"
[00:55:44] ------------------------------------------
[00:55:44] 
[00:55:44] ------------------------------------------
[00:55:44] stderr:
[00:55:44] stderr:
[00:55:44] ------------------------------------------
[00:55:44] {"message":"unreachable pattern","code":{"code":"unreachable_patterns","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/ctfe/references.rs","byte_start":868,"byte_end":871,"line_start":33,"line_end":33,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        BOO => panic!(),","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unreachable_patterns)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unreachable pattern\n  --> /checkout/src/test/run-pass/ctfe/references.rs:33:9\n   |\nLL |         BOO => panic!(),\n   |         ^^^\n   |\n   = note: #[warn(unreachable_patterns)] on by default\n\n"}
[00:55:44] ------------------------------------------
[00:55:44] 
[00:55:44] thread '[run-pass] run-pass/ctfe/references.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:55:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
