plain
travis_time:end:0bcf0130:start=1553637869470557489,finish=1553637958374163292,duration=88903605803
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:11:36] .............................i...................................................................... 4700/5497
[01:11:42] .................................................................................................... 4800/5497
[01:11:45] .................................................................................................... 4900/5497
[01:11:49] .................................................................................................... 5000/5497
[01:11:53] .............................................................F...................................... 5100/5497
[01:12:00] .................................................................................................... 5300/5497
[01:12:03] .................................................................................................... 5400/5497
[01:12:06] ...................................i.............................................................
[01:12:06] failures:
---
[01:12:06] 1 error[E0034]: multiple applicable items in scope
[01:12:06] -   --> $DIR/trait-alias-ambiguous.rs:16:9
[01:12:06] +   --> $DIR/trait-alias-ambiguous.rs:21:7
[01:12:06] 3    |
[01:12:06] 4 LL |     t.foo();
[01:12:06] 5    |       ^^^ multiple `foo` found
[01:12:06] 
[01:12:06] - note: candidate #1 is defined in the trait `A`
[01:12:06] -   --> $DIR/trait-alias-ambiguous.rs:4:19
[01:12:06] 8    |
[01:12:06] - LL |     pub trait A { fn foo(&self); }
[01:12:06] -    |                   ^^^^^^^^^^^^^^
[01:12:06] -    = help: to disambiguate the method call, write `A::foo(t)` instead
[01:12:06] - note: candidate #2 is defined in the trait `B`
[01:12:06] -   --> $DIR/trait-alias-ambiguous.rs:5:19
[01:12:06] + note: candidate #1 is defined in an impl of the trait `inner::A` for the type `u8`
[01:12:06] +   --> $DIR/trait-alias-ambiguous.rs:8:9
[01:12:06] 14    |
[01:12:06] - LL |     pub trait B { fn foo(&self); }
[01:12:06] -    |                   ^^^^^^^^^^^^^^
[01:12:06] -    = help: to disambiguate the method call, write `B::foo(t)` instead
[01:12:06] + LL |         fn foo(&self) {}
[01:12:06] +    |         ^^^^^^^^^^^^^
[01:12:06] + note: candidate #2 is defined in an impl of the trait `inner::B` for the type `u8`
[01:12:06] +   --> $DIR/trait-alias-ambiguous.rs:11:9
[01:12:06] +    |
[01:12:06] + LL |         fn foo(&self) {}
[01:12:06] 18 
[01:12:06] 19 error: aborting due to previous error
[01:12:06] 20 
[01:12:06] 
[01:12:06] 
[01:12:06] 
[01:12:06] The actual stderr differed from the expected stderr.
[01:12:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/trait-alias-ambiguous.stderr
[01:12:06] To update references, rerun the tests and pass the `--bless` flag
[01:12:06] To only update this specific test, also pass `--test-args traits/trait-alias-ambiguous.rs`
[01:12:06] error: 1 errors occurred comparing output.
[01:12:06] status: exit code: 1
[01:12:06] status: exit code: 1
[01:12:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/auxiliary" "-A" "unused"
[01:12:06] ------------------------------------------
[01:12:06] 
[01:12:06] ------------------------------------------
[01:12:06] stderr:
[01:12:06] stderr:
[01:12:06] ------------------------------------------
[01:12:06] {"message":"multiple applicable items in scope","code":{"code":"E0034","explanation":"\nThe compiler doesn't know what method to call because more than one method\nhas the same prototype. Erroneous code example:\n\n