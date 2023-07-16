plain
travis_time:end:01020f98:start=1547392264596348508,finish=1547392352039226742,duration=87442878234
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:00:59] .................................................................................................... 200/5298
[01:01:02] .................................................................................................... 300/5298
[01:01:05] .................................................................................................... 400/5298
[01:01:08] .................................................................................................... 500/5298
[01:01:12] ..............................i.............................................FF...................... 600/5298
[01:01:20] .................................................................................................... 800/5298
[01:01:26] ........................................................................i...............i........... 900/5298
[01:01:29] ..................................................................................................ii 1000/5298
[01:01:33] iii................................................................................................. 1100/5298
---
[01:03:54] 
[01:03:54] ---- [ui] ui/coherence/coherence-impl-trait-for-marker-trait-negative.rs stdout ----
[01:03:54] diff of stderr:
[01:03:54] 
[01:03:54] 16 LL | impl !Send for dyn Marker2 {} //~ ERROR E0117
[01:03:54] 17    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
[01:03:54] 18    |
[01:03:54] -    = note: the impl does not reference any types defined in this crate
[01:03:54] +    = note: the impl does not reference only types defined in this crate
[01:03:54] 20    = note: define and implement a trait or new type instead
[01:03:54] 21 
[01:03:54] 22 error[E0321]: cross-crate traits with a default impl, like `std::marker::Send`, can only be implemented for a struct/enum type, not `(dyn Object + 'static)`
[01:03:54] 
[01:03:54] The actual stderr differed from the expected stderr.
[01:03:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative/coherence-impl-trait-for-marker-trait-negative.stderr
[01:03:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative/coherence-impl-trait-for-marker-trait-negative.stderr
[01:03:54] To update references, rerun the tests and pass the `--bless` flag
[01:03:54] To only update this specific test, also pass `--test-args coherence/coherence-impl-trait-for-marker-trait-negative.rs`
[01:03:54] error: 1 errors occurred comparing output.
[01:03:54] status: exit code: 1
[01:03:54] status: exit code: 1
[01:03:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative/auxiliary" "-A" "unused"
[01:03:54] ------------------------------------------
[01:03:54] 
[01:03:54] ------------------------------------------
[01:03:54] stderr:
[01:03:54] stderr:
[01:03:54] ------------------------------------------
[01:03:54] {"message":"the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker1`","code":{"code":"E0371","explanation":"\nWhen `Trait2` is a subtrait of `Trait1` (for example, when `Trait2` has a\ndefinition like `trait Trait2: Trait1 { ... }`), it is not allowed to implement\n`Trait1` for `Trait2`. This is because `Trait2` already implements `Trait1` by\ndefinition, so it is not useful to do this.\n\nExample:\n\n