plain
travis_time:end:09aade9a:start=1541390571584586895,finish=1541390572605767730,duration=1021180835
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:23] .................................................................................................... 2000/4989
[00:47:26] .................................................................................................... 2100/4989
[00:47:30] .................................................................................................... 2200/4989
[00:47:34] .................................................................................................... 2300/4989
[00:47:37] ......................................F............................................................. 2400/4989
[00:47:41] .......................................F..................................................F......... 2500/4989
[00:47:44] ..............F.............................................................FFFF.................... 2600/4989
[00:47:51] .................................................................................................... 2800/4989
[00:47:53] .................................................................................................... 2900/4989
[00:47:57] .................................................................................................... 3000/4989
[00:48:00] ................i................................................................................... 3100/4989
---
[00:48:54] 1 error[E0658]: statements in constants are unstable (see issue #48821)
[00:48:54] -   --> $DIR/issue32829.rs:17:9
[00:48:54] +   --> $DIR/issue-32829-2.rs:17:9
[00:48:54] 3    |
[00:48:54] 4 LL |         5;
[00:48:54] 
[00:48:54] 7    = help: add #![feature(const_let)] to the crate attributes to enable
[00:48:54] 8 
[00:48:54] 8 
[00:48:54] 9 error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
[00:48:54] +   --> $DIR/issue-32829-2.rs:25:9
[00:48:54] 11    |
[00:48:54] 11    |
[00:48:54] 12 LL |         invalid();
[00:48:54] 
[00:48:54] 14 
[00:48:54] 15 error[E0658]: statements in constants are unstable (see issue #48821)
[00:48:54] -   --> $DIR/issue32829.rs:25:9
[00:48:54] -   --> $DIR/issue32829.rs:25:9
[00:48:54] +   --> $DIR/issue-32829-2.rs:25:9
[00:48:54] 17    |
[00:48:54] 18 LL |         invalid();
[00:48:54] 
[00:48:54] 21    = help: add #![feature(const_let)] to the crate attributes to enable
[00:48:54] 22 
[00:48:54] 23 error[E0658]: statements in constants are unstable (see issue #48821)
[00:48:54] 23 error[E0658]: statements in constants are unstable (see issue #48821)
[00:48:54] -   --> $DIR/issue32829.rs:34:9
[00:48:54] +   --> $DIR/issue-32829-2.rs:34:9
[00:48:54] 25    |
[00:48:54] 26 LL |         valid();
[00:48:54] 
[00:48:54] 29    = help: add #![feature(const_let)] to the crate attributes to enable
[00:48:54] 30 
[00:48:54] 31 error[E0658]: statements in statics are unstable (see issue #48821)
[00:48:54] 31 error[E0658]: statements in statics are unstable (see issue #48821)
[00:48:54] -   --> $DIR/issue32829.rs:42:9
[00:48:54] +   --> $DIR/issue-32829-2.rs:42:9
[00:48:54] 33    |
[00:48:54] 34 LL |         5;
[00:48:54] 
[00:48:54] 37    = help: add #![feature(const_let)] to the crate attributes to enable
[00:48:54] 38 
[00:48:54] 38 
[00:48:54] 39 error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
[00:48:54] +   --> $DIR/issue-32829-2.rs:50:9
[00:48:54] 41    |
[00:48:54] 41    |
[00:48:54] 42 LL |         invalid();
[00:48:54] 
[00:48:54] 44 
[00:48:54] 45 error[E0658]: statements in statics are unstable (see issue #48821)
[00:48:54] -   --> $DIR/issue32829.rs:50:9
[00:48:54] -   --> $DIR/issue32829.rs:50:9
[00:48:54] +   --> $DIR/issue-32829-2.rs:50:9
[00:48:54] 47    |
[00:48:54] 48 LL |         invalid();
[00:48:54] 
[00:48:54] 51    = help: add #![feature(const_let)] to the crate attributes to enable
[00:48:54] 52 
[00:48:54] 53 error[E0658]: statements in statics are unstable (see issue #48821)
[00:48:54] 53 error[E0658]: statements in statics are unstable (see issue #48821)
[00:48:54] -   --> $DIR/issue32829.rs:59:9
[00:48:54] +   --> $DIR/issue-32829-2.rs:59:9
[00:48:54] 55    |
[00:48:54] 56 LL |         valid();
[00:48:54] 
[00:48:54] 59    = help: add #![feature(const_let)] to the crate attributes to enable
[00:48:54] 60 
[00:48:54] 61 error[E0658]: statements in statics are unstable (see issue #48821)
[00:48:54] 61 error[E0658]: statements in statics are unstable (see issue #48821)
[00:48:54] -   --> $DIR/issue32829.rs:67:9
[00:48:54] +   --> $DIR/issue-32829-2.rs:67:9
[00:48:54] 63    |
[00:48:54] 64 LL |         5;
[00:48:54] 
[00:48:54] 67    = help: add #![feature(const_let)] to the crate attributes to enable
[00:48:54] 68 
[00:48:54] 68 
[00:48:54] 69 error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
[00:48:54] +   --> $DIR/issue-32829-2.rs:75:9
[00:48:54] 71    |
[00:48:54] 71    |
[00:48:54] 72 LL |         invalid();
[00:48:54] 
[00:48:54] 74 
[00:48:54] 75 error[E0658]: statements in statics are unstable (see issue #48821)
[00:48:54] -   --> $DIR/issue32829.rs:75:9
[00:48:54] -   --> $DIR/issue32829.rs:75:9
[00:48:54] +   --> $DIR/issue-32829-2.rs:75:9
[00:48:54] 77    |
[00:48:54] 78 LL |         invalid();
[00:48:54] 
[00:48:54] 81    = help: add #![feature(const_let)] to the crate attributes to enable
[00:48:54] 82 
[00:48:54] 83 error[E0658]: statements in statics are unstable (see issue #48821)
[00:48:54] 83 error[E0658]: statements in statics are unstable (see issue #48821)
[00:48:54] -   --> $DIR/issue32829.rs:84:9
[00:48:54] +   --> $DIR/issue-32829-2.rs:84:9
[00:48:54] 85    |
[00:48:54] 86 LL |         valid();
[00:48:54] 
[00:48:54] 
[00:48:54] The actual stderr differed from the expected stderr.
[00:48:54] The actual stderr differed from the expected stderr.
[00:48:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32829-2/issue-32829-2.stderr
[00:48:54] To update references, rerun the tests and pass the `--bless` flag
[00:48:54] To only update this specific test, also pass `--test-args issues/issue-32829-2.rs`
[00:48:54] error: 1 errors occurred comparing output.
[00:48:54] status: exit code: 1
[00:48:54] status: exit code: 1
[00:48:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32829-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32829-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32829-2/auxiliary" "-A" "unused"
[00:48:54] stdout:
[00:48:54] --------------------------------s in constants are limited to constant functions, tuple structs and tuple variants","code":{"code":"E0015","explanation":"\nThe only functions that can be called in static or constant expressions are\n`const` functions, and struct/enum constructors. `const` functions are only\navailable on a nightly compiler. Rust currently does not support more general\ncompile-time function execution.\n\n