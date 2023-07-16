plain
travis_time:end:1b7c42bc:start=1542295205547712327,finish=1542295208531036195,duration=2983323868
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:39] .................................................................................................... 4500/5019
[00:51:43] .......................................................................................i............ 4600/5019
[00:51:46] .................................................................................................... 4700/5019
[00:51:50] .................................................................................................... 4800/5019
[00:51:53] ..............F..................................................................................... 4900/5019
[00:51:55] ..........................................................i......................................... 5000/5019
rror","spans":[{"file_name":"/checkout/src/test/ui/underscore_const_names_feature_gate.rs","byte_start":472,"byte_end":489,"line_start":11,"line_end":11,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"const _: () = (); //~ ERROR is unstable","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(underscore_const_names)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: naming constants with `_` is unstable (see issue #54912)\n  --> /checkout/src/test/ui/underscore_const_names_feature_gate.rs:11:1\n   |\nLL | const _: () = (); //~ ERROR is unstable\n   | ^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(underscore_const_names)] to the crate attributes to enable\n\n"}
[00:51:56] {"message":"naming constants with `_` is unstable (see issue #54912)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n