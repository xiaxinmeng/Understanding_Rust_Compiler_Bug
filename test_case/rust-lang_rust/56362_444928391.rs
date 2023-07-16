plain
travis_time:end:00983100:start=1544109975084152681,finish=1544109976866347578,duration=1782194897
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:44] .................................................................................................... 3500/5123
[00:46:47] ...........................ii....................................................................... 3600/5123
[00:46:49] .............................................i...................................................... 3700/5123
[00:46:50] .................................................................................................... 3800/5123
[00:46:51] .i.................................................................................................. 3900/5123
[00:46:56] ...........................F........................................................................ 4000/5123
[00:47:04] .................................................................................................... 4200/5123
[00:47:07] ................................................................................................i... 4300/5123
[00:47:12] .................................................................................................... 4400/5123
[00:47:16] .................................................................................................... 4500/5123
[00:47:16] .................................................................................................... 4500/5123
[00:47:18] .................................................................................................... 4600/5123
[00:47:22] ..............................................................................i..................... 4700/5123
[00:47:25] .................................................................................................... 4800/5123
[00:47:28] .................................................................................................... 4900/5123
[00:47:31] .................................................................................................... 5000/5123
[00:47:33] ..............................................................i..................................... 5100/5123
 To only update this specific test, also pass `--test-args precise_pointer_size_matching.rs`
[00:47:34] error: 1 errors occurred comparing output.
[00:47:34] status: exit code: 1
[00:47:34] status: exit code: 1
[00:47:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/precise_pointer_size_matching.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/precise_pointer_size_matching/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/precise_pointer_size_matching/auxiliary" "-A" "unused"
[00:47:34] ------------------------------------------
[00:47:34] 
[00:47:34] ------------------------------------------
[00:47:34] stderr:
[00:47:34] stderr:
[00:47:34] ------------------------------------------
[00:47:34] {"message":"non-exhaustive patterns: `-9223372036854775808isize..=-6isize` and `21isize..=9223372036854775807isize` not covered","code":{"code":"E0004","explanation":"\nThis error indicates that the compiler cannot guarantee a matching pattern for\none or more possible inputs to a match expression. Guaranteed matches are\nrequired in order to assign values to match expressions, or alternatively,\ndetermine the flow of execution. Erroneous code example:\n\n