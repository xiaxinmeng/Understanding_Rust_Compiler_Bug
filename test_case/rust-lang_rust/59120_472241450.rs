plain
travis_time:end:0738176a:start=1552434886965788641,finish=1552434986150412312,duration=99184623671
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:16:28] .................................................................................................... 2600/5450
[01:16:33] .................................................................................................... 2700/5450
[01:16:38] .................................................................................................... 2800/5450
[01:16:42] .................................................................................................... 2900/5450
[01:16:47] .....................................F.............................................................. 3000/5450
[01:16:55] .................................................................................................... 3200/5450
[01:16:59] ...........................................................................i........................ 3300/5450
[01:17:03] .................................................................................................... 3400/5450
[01:17:07] .................................................ii...i..ii......................................... 3500/5450
---
[01:18:15] .................................................................................................... 5000/5450
[01:18:19] .................................................................................................... 5100/5450
[01:18:23] .................................................................................................... 5200/5450
[01:18:27] .................................................................................................... 5300/5450
[01:18:30] ..............................................F.........................................i........... 5400/5450
[01:18:31] failures:
[01:18:31] 
[01:18:31] ---- [ui] ui/issues/issue-56411.rs stdout ----
[01:18:31] diff of stderr:
[01:18:31] diff of stderr:
[01:18:31] 
[01:18:31] 1 error[E0255]: the name `issue_56411_aux` is defined multiple times
[01:18:31] +   --> $DIR/issue-56411.rs:5:21
[01:18:31] 3    |
[01:18:31] 3    |
[01:18:31] 4 LL |             mod $name;
[01:18:31] 5    |             ---------- previous definition of the module `issue_56411_aux` here
[01:18:31] 
[01:18:31] 15    = note: `issue_56411_aux` must be defined only once in the type namespace of this module
[01:18:31] 16 
[01:18:31] 17 error[E0365]: `issue_56411_aux` is private, and cannot be re-exported
[01:18:31] +   --> $DIR/issue-56411.rs:5:21
[01:18:31] 19    |
[01:18:31] 19    |
[01:18:31] 20 LL |             pub use self::$name;
[01:18:31] 21    |                     ^^^^^^^^^^^ re-export of private `issue_56411_aux`
[01:18:31] 
[01:18:31] The actual stderr differed from the expected stderr.
[01:18:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/issue-56411.stderr
[01:18:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/issue-56411.stderr
[01:18:31] To update references, rerun the tests and pass the `--bless` flag
[01:18:31] To only update this specific test, also pass `--test-args issues/issue-56411.rs`
[01:18:31] error: 1 errors occurred comparing output.
[01:18:31] status: exit code: 1
[01:18:31] status: exit code: 1
[01:18:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-56411.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/auxiliary" "-A" "unused"
[01:18:31] ------------------------------------------
[01:18:31] 
[01:18:31] ------------------------------------------
[01:18:31] stderr:
[01:18:31] stderr:
[01:18:31] ------------------------------------------
[01:18:31] {"message":"the name `issue_56411_aux` is defined multiple times","code":{"code":"E0255","explanation":"\nYou can't import a value whose name is the same as another value defined in the\nmodule.\n\nErroneous code example:\n\n