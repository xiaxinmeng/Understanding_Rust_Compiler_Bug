plain
travis_time:end:1cff5dee:start=1541042911170445266,finish=1541042966331713766,duration=55161268500
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:31] .................................................................................................... 500/4984
[00:48:34] ........................i........................................................................... 600/4984
[00:48:39] .................................................................................................... 700/4984
[00:48:45] ..................................................................i...........i..................... 800/4984
[00:48:48] ...F.................................................................................iiiii.......... 900/4984
[00:48:54] .................................................................................................... 1100/4984
[00:48:56] .................................................................................................... 1200/4984
[00:48:59] .................................................................................................... 1300/4984
[00:49:01] .................................................................................................... 1400/4984
---
[00:50:49] .......................................................i............................................ 4600/4984
[00:50:52] .................................................................................................... 4700/4984
00] +   --> $DIR/derive-uninhabited-enum-38885.rs:21:1
[00:51:00] 11    |
[00:51:00] 12 LL | enum Foo { //~ WARN never used
[00:51:00] 
[00:51:00] 
[00:51:00] The actual stderr differed from the expected stderr.
[00:51:00] The actual stderr differed from the expected stderr.
[00:51:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derive-uninhabited-enum-38885/derive-uninhabited-enum-38885.stderr
[00:51:00] To update references, rerun the tests and pass the `--bless` flag
[00:51:00] To only update this specific test, also pass `--test-args derive-uninhabited-enum-38885.rs`
[00:51:00] error: 1 errors occurred comparing output.
[00:51:00] status: exit code: 0
[00:51:00] status: exit code: 0
[00:51:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derive-uninhabited-enum-38885.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derive-uninhabited-enum-38885/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Wunused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derive-uninhabited-enum-38885/auxiliary" "-A" "unused"
[00:51:00] ------------------------------------------
[00:51:00] 
[00:51:00] ------------------------------------------
[00:51:00] stderr:
[00:51:00] stderr:
[00:51:00] ------------------------------------------
[00:51:00] {"message":"enum is never used: `Void`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/derive-uninhabit4743480 .
2476256 ./obj/build
1847548 ./obj/build/x86_64-unknown-linux-gnu
1194432 ./.git
1072228 ./src
