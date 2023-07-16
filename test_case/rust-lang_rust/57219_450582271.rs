plain
travis_time:end:02f0ea70:start=1546195429452312034,finish=1546195485530191743,duration=56077879709
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:24] .................................................................................................... 4200/5210
[00:59:27] .................................................................................................... 4300/5210
[00:59:30] ....................................................................i............................... 4400/5210
[00:59:36] .................................................................................................... 4500/5210
[00:59:39] ..............................................................................F..................... 4600/5210
[00:59:46] .................................................................................................... 4800/5210
[00:59:49] .................................................................................................... 4900/5210
[00:59:52] .................................................................................................... 5000/5210
[00:59:52] .................................................................................................... 5000/5210
pck_vec_cycle_checked.rs:102:25\n   |\nLL |     c1.v[1].v.set(Some(&c3));\n   |                         ^^ borrowed value does not live long enough\n...\nLL | }\n   | - `c3` dropped here while still borrowed\n   |\n   = note: values in a scope are dropped in the opposite order they are created\n\n"}
[00:59:58] {"message":"`c2` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n