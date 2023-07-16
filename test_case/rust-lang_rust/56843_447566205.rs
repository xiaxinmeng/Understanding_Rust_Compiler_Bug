plain
travis_time:end:12acb1b0:start=1544875305952488894,finish=1544875361166895463,duration=55214406569
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:19] .................................................................................................... 4600/5171
[00:46:22] .................................................................................................... 4700/5171
[00:46:26] ..........................i......................................................................... 4800/5171
[00:46:30] .................................................................................................... 4900/5171
[00:46:35] .................................................F.................................................. 5100/5171
e/use-after-move-self/use-after-move-self.stderr
[00:46:37] To update references, rerun the tests and pass the `--bless` flag
[00:46:37] To only update this specific test, also pass `--test-args use/use-after-move-self.rs`
[00:46:37] error: 1 errors occurred comparing output.
[00:46:37] status: exit code: 1
[00:46:37] status: exit code: 1
[00:46:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/use/use-after-move-self.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-after-move-self/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-after-move-self/auxiliary" "-A" "unused"
[00:46:37] ------------------------------------------
[00:46:37] 
[00:46:37] ------------------------------------------
[00:46:37] stderr:
[00:46:37] stderr:
[00:46:37] ------------------------------------------
[00:46:37] {"message":"use of moved value: `self`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n