plain
travis_time:end:15509fce:start=1544037119505303600,finish=1544037121753155259,duration=2247851659
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:48] .................................................................................................... 4400/5110
[00:46:51] .................................................................................................... 4500/5110
[00:46:54] .................................................................................................... 4600/5110
[00:46:58] .........................................................................i.......................... 4700/5110
[00:47:00] ...................F................................................................................ 4800/5110
[00:47:06] .................................................................................................... 5000/5110
[00:47:09] .................................................i.................................................. 5100/5110
 more information, see issue #56522 <https://github.com/rust-lang/rust/issues/56522>
[00:47:09] + 
[00:47:09] + 
[00:47:09] 1 error[E0592]: duplicate definitions with name `test`
[00:47:09] 2   --> $DIR/trait-object-auto-dedup-in-impl.rs:24:5
[00:47:09] 3    |
[00:47:09] 
[00:47:09] 
[00:47:09] The actual stderr differed from the expected stderr.
[00:47:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-auto-dedup-in-impl/trait-object-auto-dedup-in-impl.stderr
[00:47:09] To update references, rerun the tests and pass the `--bless` flag
[00:47:09] To only update this specific test, also pass `--test-args traits/trait-object-auto-dedup-in-impl.rs`
[00:47:09] error: 1 errors occurred comparing output.
[00:47:09] status: exit code: 1
[00:47:09] status: exit code: 1
[00:47:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-auto-dedup-in-impl.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-auto-dedup-in-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-auto-dedup-in-impl/auxiliary" "-A" "unused"
[00:47:09] ------------------------------------------
[00:47:09] 
[00:47:09] ------------------------------------------
[00:47:09] stderr:
[00:47:09] stderr:
[00:47:09] ------------------------------------------
[00:47:09] {"message":"duplicate auto trait `
