plain
travis_time:end:0d9ee38e:start=1541194990057349859,finish=1541195061059559613,duration=71002209754
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:56] .................................................................................................... 3700/4983
[00:48:57] ......................i............................................................................. 3800/4983
[00:49:00] .................................................................................................... 3900/4983
[00:49:03] .................................................................................................... 4000/4983
[00:49:06] ...........................................................................................F........ 4100/4983
[00:49:14] .................................................................................................... 4300/4983
[00:49:18] .................................................................................................... 4400/4983
[00:49:20] .................................................................................................... 4500/4983
[00:49:24] ......................................................i............................................. 4600/4983
[00:49:24] ......................................................i............................................. 4600/4983
[00:49:28] .................................................................................................... 4700/4983
[00:49:31] .................................................................................................... 4800/4983
[00:49:33] .................................................................................................... 4900/4983
35] +    |                                 ^^^^^^^^
[00:49:35] + 
[00:49:35] + error[E0691]: zero-sized field in transparent struct has alignment larger than 1
[00:49:35] +   --> $DIR/repr-transparent.rs:60:30
[00:49:35] +    |
[00:49:35] + LL | struct GenericAlignExtern<T>(ZstAlign32<T>, ExternType); //~ ERROR alignment larger than 1
[00:49:35] + 
[00:49:35] + error: aborting due to 10 previous errors
[00:49:35] 70 
[00:49:35] 71 Some errors occurred: E0690, E0691.
[00:49:35] 71 Some errors occurred: E0690, E0691.
[00:49:35] 72 For more information about an error, try `rustc --explain E0690`.
[00:49:35] 
[00:49:35] 
[00:49:35] The actual stderr differed from the expected stderr.
[00:49:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-transparent/repr-transparent.stderr
[00:49:35] To update references, rerun the tests and pass the `--bless` flag
[00:49:35] To only update this specific test, also pass `--test-args repr/repr-transparent.rs`
[00:49:35] error: 1 errors occurred comparing output.
[00:49:35] status: exit code: 1
[00:49:35] status: exit code: 1
[00:49:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repr/repr-transparent.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-transparent/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-transparent/auxiliary" "-A" "eld, but has 2\n    unit: U,\n}\n