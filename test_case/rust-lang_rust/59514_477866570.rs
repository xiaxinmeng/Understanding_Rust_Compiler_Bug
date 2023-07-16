plain
travis_time:end:001168ec:start=1553830693456851826,finish=1553830695802015183,duration=2345163357
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:09:51] 
[01:09:51] running 5501 tests
[01:09:53] .................................................................................................... 100/5501
[01:09:56] .................................................................................................... 200/5501
[01:09:59] ...........................................................F........................................ 300/5501
[01:10:06] .................................................................................................... 500/5501
[01:10:09] ..........................................i......................................................... 600/5501
[01:10:13] .................................................................................................... 700/5501
[01:10:18] .................................................................................................... 800/5501
---
[01:13:10] 
[01:13:10] 269 LL |         drop(x);
[01:13:10] 270    |              - mutable borrow later used here
[01:13:10] 271 
[01:13:10] - error[E0502]: cannot borrow `e.x` as immutable because it is also borrowed as mutable
[01:13:10] + error[E0502]: cannot borrow `e.0` as immutable because it is also borrowed as mutable
[01:13:10] 274    |
[01:13:10] 275 LL |         let x = &mut e;
[01:13:10] 
[01:13:10] 
[01:13:10] 
[01:13:10] The actual stderr differed from the expected stderr.
[01:13:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.mir/borrowck-describe-lvalue.mir.stderr
[01:13:10] To update references, rerun the tests and pass the `--bless` flag
[01:13:10] To only update this specific test, also pass `--test-args borrowck/borrowck-describe-lvalue.rs`
[01:13:10] 
[01:13:10] error in revision `mir`: 1 errors occurred comparing output.
[01:13:10] status: exit code: 1
[01:13:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.mir/auxiliary" "-A" "unused"
[01:13:10] ------------------------------------------
[01:13:10] 
[01:13:10] ------------------------------------------
[01:13:10] stderr:
[01:13:10] stderr:
[01:13:10] ------------------------------------------
[01:13:10] {"message":"cannot borrow `x` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n