plain
travis_time:end:152e3794:start=1544206089533048773,finish=1544206090621814294,duration=1088765521
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:39] .................................................................................i.................. 200/2926
[00:47:47] .................................................................................................... 300/2926
[00:47:56] .................................................................................................... 400/2926
[00:48:05] .................................................................................................... 500/2926
[00:48:16] ............................................F....................................................... 600/2926
[00:48:40] .................................................................................................... 800/2926
[00:48:49] .................................................................................................... 900/2926
[00:49:03] .................................................................................................... 1000/2926
[00:49:15] .................................................................................................... 1100/2926
---
[00:52:43] .................................................................................................... 2600/2926
[00:52:51] .................................................................................................... 2700/2926
[00:53:01] .................................................................................................... 2800/2926
[00:53:12] .................................................................................................... 2900/2926
pers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/ctfe/references/auxiliary"
[00:53:16] ------------------------------------------
[00:53:16] 
[00:53:16] ------------------------------------------
[00:53:16] stderr:
[00:53:16] stderr:
[00:53:16] ------------------------------------------
[00:53:16] {"message":"unreachable pattern","code":{"code":"unreachable_patterns","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/ctfe/references.rs","byte_start":868,"byte_end":871,"line_start":33,"line_end":33,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        BOO => panic!(),","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unreachable_patterns)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unreachable pattern\n  --> /checkout/src/test/run-pass/ctfe/references.rs:33:9\n   |\nLL |         BOO => panic!(),\n   |         ^^^\n   |\n   = note: #[warn(unreachable_patterns)] on by default\n\n"}
[00:53:16] ------------------------------------------
[00:53:16] 
[00:53:16] thread '[run-pass] run-pass/ctfe/references.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[00:53:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
