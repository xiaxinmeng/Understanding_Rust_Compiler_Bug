plain
travis_time:end:105ea708:start=1546548747018422529,finish=1546548748098536144,duration=1080113615
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:01:21] .................................................................................................... 1300/5231
[01:01:24] .................................................................................................... 1400/5231
[01:01:27] .................................................................................................... 1500/5231
[01:01:30] ...........................................i........................................................ 1600/5231
[01:01:33] ..................i..........................................................................F...... 1700/5231
[01:01:41] .................................................................................................... 1900/5231
[01:01:44] ...................................................................i................................ 2000/5231
[01:01:48] .................................................................................................... 2100/5231
[01:01:52] .................................................................................................... 2200/5231
---
[01:03:44] 
[01:03:44] ---- [ui] ui/impl-trait/infinite-impl-trait-issue-38064.rs stdout ----
[01:03:44] diff of stderr:
[01:03:44] 
[01:03:44] 1 error[E0720]: opaque type expands to a recursive type
[01:03:44] +   --> $DIR/infinite-impl-trait-issue-38064.rs:8:13
[01:03:44] 3    |
[01:03:44] 3    |
[01:03:44] 4 LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
[01:03:44] 5    |             ^^^^^^^^^ expands to self-referential type
[01:03:44] 
[01:03:44] 7    = note: expanded type is `foo::Foo<bar::Bar<impl Quux>>`
[01:03:44] 8 
[01:03:44] 9 error[E0720]: opaque type expands to a recursive type
[01:03:44] +   --> $DIR/infinite-impl-trait-issue-38064.rs:14:13
[01:03:44] 11    |
[01:03:44] 11    |
[01:03:44] 12 LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
[01:03:44] 13    |             ^^^^^^^^^ expands to self-referential type
[01:03:44] 
[01:03:44] The actual stderr differed from the expected stderr.
[01:03:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/infinite-impl-trait-issue-38064/infinite-impl-trait-issue-38064.stderr
[01:03:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/infinite-impl-trait-issue-38064/infinite-impl-trait-issue-38064.stderr
[01:03:44] To update references, rerun the tests and pass the `--bless` flag
[01:03:44] To only update this specific test, also pass `--test-args impl-trait/infinite-impl-trait-issue-38064.rs`
[01:03:44] error: 1 errors occurred comparing output.
[01:03:44] status: exit code: 1
[01:03:44] status: exit code: 1
[01:03:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/infinite-impl-trait-issue-38064.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/infinite-impl-trait-issue-38064/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/infinite-impl-trait-issue-38064/auxiliary" "-A" "unused"
[01:03:44] ------------------------------------------
[01:03:44] 
[01:03:44] ------------------------------------------
[01:03:44] stderr:
[01:03:44] stderr:
[01:03:44] ------------------------------------------
[01:03:44] {"message":"opaque type expands to a recursive type","code":{"code":"E0720","explanation":"\nAn `impl Trait` type expands to a recursive type.\n\nAn `impl Trait` type must be expandable to a concrete type that contains no\n`impl Trait` types. For example the following example tries to create an\n`impl Trait` type `T` that is equal to `[T, T]`:\n\n