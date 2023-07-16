plain
travis_time:end:03bdf460:start=1549883860523314569,finish=1549883937418204613,duration=76894890044
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:58] .................................................................................................... 2200/5380
[01:00:02] .................................................................................................... 2300/5380
[01:00:06] .................................................................................................... 2400/5380
[01:00:10] .................................................................................................... 2500/5380
[01:00:14] .............................................F...................................................... 2600/5380
[01:00:23] .................................................................................................... 2800/5380
[01:00:27] .................................................................................................... 2900/5380
[01:00:31] .................................................................................................... 3000/5380
[01:00:34] .................................................................................................... 3100/5380
---
[01:02:06] 
[01:02:06] ---- [ui] ui/issues/issue-35677.rs stdout ----
[01:02:06] diff of stderr:
[01:02:06] 
[01:02:06] - error[E0599]: no method named `drain` found for type `&mut std::collections::HashMap<K, V>` in the current scope
[01:02:06] + error[E0308]: mismatched types
[01:02:06] +   --> $DIR/issue-35677.rs:3:5
[01:02:06] 3    |
[01:02:06] 3    |
[01:02:06] + LL | fn intersect_map<K, V>(this: &mut HashMap<K, V>, other: HashMap<K, V>) -> bool {
[01:02:06] +    |                                                                           ---- expected `bool` because of return type
[01:02:06] 4 LL |     this.drain()
[01:02:06] +    |     ^^^^^^^^^^^^ expected bool, found struct `std::collections::hash_map::Drain`
[01:02:06] 6    |
[01:02:06] 6    |
[01:02:06] -    = note: the method `drain` exists but the following trait bounds were not satisfied:
[01:02:06] -            `K : std::cmp::Eq`
[01:02:06] -            `K : std::hash::Hash`
[01:02:06] +    = note: expected type `bool`
[01:02:06] +               found type `std::collections::hash_map::Drain<'_, K, V>`
[01:02:06] 11 error: aborting due to previous error
[01:02:06] 12 
[01:02:06] 
[01:02:06] - For more information about this error, try `rustc --explain E0599`.
[01:02:06] - For more information about this error, try `rustc --explain E0599`.
[01:02:06] + For more information about this error, try `rustc --explain E0308`.
[01:02:06] 14 
[01:02:06] 
[01:02:06] 
[01:02:06] The actual stderr differed from the expected stderr.
[01:02:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35677/issue-35677.stderr
[01:02:06] To update references, rerun the tests and pass the `--bless` flag
[01:02:06] To only update this specific test, also pass `--test-args issues/issue-35677.rs`
[01:02:06] error: 1 errors occurred comparing output.
[01:02:06] status: exit code: 1
[01:02:06] status: exit code: 1
[01:02:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35677.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35677/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35677/auxiliary" "-A" "unused"
[01:02:06] ------------------------------------------
[01:02:06] 
[01:02:06] ------------------------------------------
[01:02:06] stderr:
[01:02:06] stderr:
[01:02:06] ------------------------------------------
[01:02:06] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n