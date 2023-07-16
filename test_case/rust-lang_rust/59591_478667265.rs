plain
travis_time:end:22be69ea:start=1554134503030728313,finish=1554134625610442196,duration=122579713883
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:11:59] .................................................................................................... 1900/5516
[01:12:03] .................................................................................................... 2000/5516
[01:12:06] ......................................................................................i............. 2100/5516
[01:12:10] .................................................................................................... 2200/5516
[01:12:14] .............................................................F...................................... 2300/5516
[01:12:23] .................................................................................................... 2500/5516
[01:12:26] .................................................................................................... 2600/5516
[01:12:30] .................................................................................................... 2700/5516
[01:12:35] .................................................................................................... 2800/5516
[01:12:35] .................................................................................................... 2800/5516
[01:12:39] .......................................................F............................................ 2900/5516
[01:12:47] .................................................................................................... 3100/5516
[01:12:50] .................................................................................................... 3200/5516
[01:12:54] .................................................................................................... 3300/5516
[01:12:58] ..................i................................................................................. 3400/5516
---
[01:14:19] diff of stderr:
[01:14:19] 
[01:14:19] 10   --> $DIR/issue-2149.rs:13:12
[01:14:19] 11    |
[01:14:19] 12 LL |     ["hi"].bind(|x| [x] );
[01:14:19] +    |            ^^^^ help: did you mean: `find`
[01:14:19] 14    |
[01:14:19] 15    = help: items from traits can only be used if the trait is implemented and in scope
[01:14:19] 15    = help: items from traits can only be used if the trait is implemented and in scope
[01:14:19] 16    = note: the following trait defines an item `bind`, perhaps you need to implement it:
[01:14:19] 
[01:14:19] The actual stderr differed from the expected stderr.
[01:14:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2149/issue-2149.stderr
[01:14:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2149/issue-2149.stderr
[01:14:19] To update references, rerun the tests and pass the `--bless` flag
[01:14:19] To only update this specific test, also pass `--test-args issues/issue-2149.rs`
[01:14:19] error: 1 errors occurred comparing output.
[01:14:19] status: exit code: 1
[01:14:19] status: exit code: 1
[01:14:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2149.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2149/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2149/auxiliary" "-A" "unused"
[01:14:19] ------------------------------------------
[01:14:19] 
[01:14:19] ------------------------------------------
[01:14:19] stderr:
[01:14:19] stderr:
[01:14:19] ------------------------------------------
[01:14:19] {"message":"cannot add `std::vec::Vec<B>` to `()`","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n