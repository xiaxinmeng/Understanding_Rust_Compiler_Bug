plain
travis_time:end:1f59a500:start=1542018322998767874,finish=1542018324182782580,duration=1184014706
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:41] .................................................................................................... 2400/5014
[00:48:45] .................................................................................................... 2500/5014
[00:48:49] .................................................................................................... 2600/5014
[00:48:53] .................................................................................................... 2700/5014
[00:48:56] .............F...................................................................................... 2800/5014
[00:49:03] .................................................................................................... 3000/5014
[00:49:06] ....................................i............................................................... 3100/5014
[00:49:09] ...................................................................................................i 3200/5014
[00:49:12] .i..ii.............................................................................................. 3300/5014
---
[00:50:00] .................................................................................................... 4900/5014
[00:50:03] .....................................................i.............................................. 5000/5014
put.
[00:50:03] status: exit code: 1
[00:50:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52023-array-size-pointer-cast.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52023-array-size-pointer-cast/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52023-array-size-pointer-cast/auxiliary" "-A" "unused"
[00:50:03] ------------------------------------------
[00:50:03] 
[00:50:03] ------------------------------------------
[00:50:03] stderr:
[00:50:03] stderr:
[00:50:03] ------------------------------------------
[00:50:03] {"message":"casting pointers to integers in constants is unstable (see issue #51910)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n