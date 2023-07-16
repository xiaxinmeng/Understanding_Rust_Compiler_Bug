plain
travis_time:end:03743e75:start=1552510661964021966,finish=1552510664257143648,duration=2293121682
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:14:54] .................................................................................................... 2700/5483
[01:14:59] .................................................................................................... 2800/5483
[01:15:03] .................................................................................................... 2900/5483
[01:15:08] .................................................................................................... 3000/5483
[01:15:12] ...............................................................FF................................... 3100/5483
[01:15:20] ...............................................................................................i.... 3300/5483
[01:15:24] .................................................................................................... 3400/5483
[01:15:28] .....................................................................ii...i..ii..................... 3500/5483
[01:15:33] .................................................................................................... 3600/5483
---
[01:16:51] .................................................................................................... 5400/5483
[01:16:54] ..................i................................................................
[01:16:54] failures:
[01:16:54] 
[01:16:54] ---- [ui] ui/kindck/kindck-inherited-copy-bound.rs#curr stdout ----
[01:16:54] 
[01:16:54] 
[01:16:54] 1 error[E0277]: the trait bound `std::boxed::Box<{integer}>: std::marker::Copy` is not satisfied
[01:16:54] 2   --> $DIR/kindck-inherited-copy-bound.rs:21:5
[01:16:54] 3    |
[01:16:54] - LL |     take_param(&x);
[01:16:54] + LL |     take_param(&x); //[curr]~ ERROR E0277
[01:16:54] 5    |     ^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `std::boxed::Box<{integer}>`
[01:16:54] 6    |
[01:16:54] 7    = note: required because of the requirements on the impl of `Foo` for `std::boxed::Box<{integer}>`
[01:16:54] 
[01:16:54] The actual stderr differed from the expected stderr.
[01:16:54] The actual stderr differed from the expected stderr.
[01:16:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-inherited-copy-bound.curr/kindck-inherited-copy-bound.curr.stderr
[01:16:54] To update references, rerun the tests and pass the `--bless` flag
[01:16:54] To only update this specific test, also pass `--test-args kindck/kindck-inherited-copy-bound.rs`
[01:16:54] 
[01:16:54] error in revision `curr`: 1 errors occurred comparing output.
[01:16:54] status: exit code: 1
[01:16:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-inherited-copy-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "curr" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-inherited-copy-bound.curr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-inherited-copy-bound.curr/auxiliary" "-A" "unused"
[01:16:54] ------------------------------------------
[01:16:54] 
[01:16:54] ------------------------------------------
[01:16:54] stderr:
[01:16:54] stderr:
[01:16:54] ------------------------------------------
[01:16:54] {"message":"the trait bound `std::boxed::Box<{integer}>: std::marker::Copy` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n