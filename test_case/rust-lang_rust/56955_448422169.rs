plain
travis_time:end:1c643dbd:start=1545175885275744852,finish=1545175942557492207,duration=57281747355
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:46:55] .................................................................................................... 2200/5187
[00:46:58] .................................................................................................... 2300/5187
[00:47:02] .................................................................................................... 2400/5187
[00:47:06] .................................................................................................... 2500/5187
[00:47:09] ..................................................................F................................. 2600/5187
[00:47:17] .................................................................................................... 2800/5187
[00:47:20] .................................................................................................... 2900/5187
[00:47:24] .................................................................................................... 3000/5187
[00:47:27] ............................................................................................i....... 3100/5187
---
[00:48:39] 
[00:48:39] ---- [ui] ui/issues/issue-41974.rs stdout ----
[00:48:39] diff of stderr:
[00:48:39] 
[00:48:39] - error[E0119]: conflicting implementations of trait `std::ops::Drop` for type `std::boxed::Box<_>`:
[00:48:39] + error[E0119]: conflicting implementations of trait `std::ops::Drop` for type `std::sync::Arc<_>`:
[00:48:39] 3    |
[00:48:39] 3    |
[00:48:39] 4 LL | impl<T> Drop for T where T: A { //~ ERROR E0119
[00:48:39] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:48:39] 6    |
[00:48:39] 7    = note: conflicting implementation in crate `alloc`:
[00:48:39] 7    = note: conflicting implementation in crate `alloc`:
[00:48:39] -            - impl<T> std::ops::Drop for std::boxed::Box<T>
[00:48:39] +            - impl<T> std::ops::Drop for std::sync::Arc<T>
[00:48:39] 9              where T: ?Sized;
[00:48:39] -    = note: downstream crates may implement trait `A` for type `std::boxed::Box<_>`
[00:48:39] +    = note: downstream crates may implement trait `A` for type `std::sync::Arc<_>`
[00:48:39] 11 
[00:48:39] 12 error[E0120]: the Drop trait may only be implemented on structures
[00:48:39] 
[00:48:39] 
[00:48:39] The actual stderr differed from the expected stderr.
[00:48:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974/issue-41974.stderr
[00:48:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974/issue-41974.stderr
[00:48:39] To update references, rerun the tests and pass the `--bless` flag
[00:48:39] To only update this specific test, also pass `--test-args issues/issue-41974.rs`
[00:48:39] error: 1 errors occurred comparing output.
[00:48:39] status: exit code: 1
[00:48:39] status: exit code: 1
[00:48:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41974.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974/auxiliary" "-A" "unused"
[00:48:39] ------------------------------------------
[00:48:39] 
[00:48:39] ------------------------------------------
[00:48:39] stderr:
[00:48:39] stderr:
[00:48:39] ------------------------------------------
[00:48:39] {"message":"conflicting implementations of trait `std::ops::Drop` for type `std::sync::Arc<_>`:","code":{"code":"E0119","explanation":"\nThere are conflicting trait implementations for the same type.\nExample of erroneous code:\n\n