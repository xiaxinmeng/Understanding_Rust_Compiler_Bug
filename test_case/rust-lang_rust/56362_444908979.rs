plain
travis_time:end:00e1128a:start=1544106980825711898,finish=1544107035779812663,duration=54954100765
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:45:50] .................................................................................................... 3500/5123
[00:45:53] ...........................ii....................................................................... 3600/5123
[00:45:55] .............................................i...................................................... 3700/5123
[00:45:56] .................................................................................................... 3800/5123
[00:45:57] .i.................................................................................................. 3900/5123
[00:46:02] ...........................F........................................................................ 4000/5123
[00:46:09] .................................................................................................... 4200/5123
[00:46:13] ................................................................................................i... 4300/5123
[00:46:17] .................................................................................................... 4400/5123
[00:46:21] .................................................................................................... 4500/5123
---
[00:46:39] 
[00:46:39] ---- [ui] ui/precise_pointer_size_matching.rs stdout ----
[00:46:39] diff of stderr:
[00:46:39] 
[00:46:39] 1 error[E0004]: non-exhaustive patterns: `$ISIZE_MIN..=-6isize` and `21isize..=$ISIZE_MAX` not covered
[00:46:39] +   --> $DIR/precise_pointer_size_matching.rs:27:11
[00:46:39] 3    |
[00:46:39] 3    |
[00:46:39] 4 LL |     match 0isize { //~ ERROR non-exhaustive patterns
[00:46:39] 5    |           ^^^^^^ patterns `$ISIZE_MIN..=-6isize` and `21isize..=$ISIZE_MAX` not covered
[00:46:39] 6 
[00:46:39] 6 
[00:46:39] 7 error[E0004]: non-exhaustive patterns: `0usize` and `21usize..=$USIZE_MAX` not covered
[00:46:39] +   --> $DIR/precise_pointer_size_matching.rs:32:11
[00:46:39] 9    |
[00:46:39] 9    |
[00:46:39] 10 LL |     match 0usize { //~ ERROR non-exhaustive patterns
[00:46:39] 11    |           ^^^^^^ patterns `0usize` and `21usize..=$USIZE_MAX` not covered
[00:46:39] 
[00:46:39] The actual stderr differed from the expected stderr.
[00:46:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/precise_pointer_size_matching/precise_pointer_size_matching.stderr
[00:46:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/precise_pointer_size_matching/precise_pointer_size_matching.stderr
[00:46:39] To update references, rerun the tests and pass the `--bless` flag
[00:46:39] To only update this specific test, also pass `--test-args precise_pointer_size_matching.rs`
[00:46:39] error: 1 errors occurred comparing output.
[00:46:39] status: exit code: 1
[00:46:39] status: exit code: 1
[00:46:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/precise_pointer_size_matching.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/precise_pointer_size_matching/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/precise_pointer_size_matching/auxiliary" "-A" "unused"
[00:46:39] ------------------------------------------
[00:46:39] 
[00:46:39] ------------------------------------------
[00:46:39] stderr:
[00:46:39] stderr:
[00:46:39] ------------------------------------------
[00:46:39] {"message":"non-exhaustive patterns: `-9223372036854775808isize..=-6isize` and `21isize..=9223372036854775807isize` not covered","code":{"code":"E0004","explanation":"\nThis error indicates that the compiler cannot guarantee a matching pattern for\none or more possible inputs to a match expression. Guaranteed matches are\nrequired in order to assign values to match expressions, or alternatively,\ndetermine the flow of execution. Erroneous code example:\n\n