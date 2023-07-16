plain
travis_time:end:2f3792ee:start=1541438200611337114,finish=1541438203249678271,duration=2638341157
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:02] .................................................................................................... 1200/4990
[00:48:05] .................................................................................................... 1300/4990
[00:48:07] .................................................................................................... 1400/4990
[00:48:10] ...................................................................................i................ 1500/4990
[00:48:13] .....F...............................................i.............................................. 1600/4990
[00:48:20] .................................................................................................... 1800/4990
[00:48:23] ..............................................................................................i..... 1900/4990
[00:48:26] .................................................................................................... 2000/4990
[00:48:30] .................................................................................................... 2100/4990
---
[00:49:50] ............................................................i....................................... 4600/4990
[00:49:54] .................................................................................................... 4700/4990
[00:49:57] .................................................................................................... 4800/4990
[00:49:59] .................................................................................................... 4900/4990
2] -    |     ^^^^^^^^^^^^ the trait `std::marker::Unpin` is not implemented for `[static generator@$DIR/static-not-unpin.rs:19:25: 21:6 _]`
[00:50:02] +    |     ^^^^^^^^^^^^ the trait `std::pin::Unpin` is not implemented for `[static generator@$DIR/static-not-unpin.rs:19:25: 21:6 _]`
[00:50:02] 6    |
[00:50:02] 7 note: required by `assert_unpin`
[00:50:02] 8   --> $DIR/static-not-unpin.rs:15:1
[00:50:02] 
[00:50:02] The actual stderr differed from the expected stderr.
[00:50:02] The actual stderr differed from the expected stderr.
[00:50:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/static-not-unpin/static-not-unpin.stderr
[00:50:02] To update references, rerun the tests and pass the `--bless` flag
[00:50:02] To only update this specific test, also pass `--test-args generator/static-not-unpin.rs`
[00:50:02] error: 1 errors occurred comparing output.
[00:50:02] status: exit code: 1
[00:50:02] status: exit code: 1
[00:50:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/static-not-unpin.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/static-not-unpin/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/static-not-unpin/auxiliary" "-A" "unused"
[00:50:02] ------------------------------------------
[00:50:02] 
[00:50:02] ------------------------------------------
[00:50:02] stderr:
[00:50:02] stderr:
[00:50:02] ------------------------------------------
[00:50:02] {"message":"the trait bound `[static generator@/checkout/src/test/ui/generator/static-not-unpin.rs:19:25: 21:6 _]: std::pin::Unpin` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n