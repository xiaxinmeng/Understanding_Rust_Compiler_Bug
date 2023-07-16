plain
travis_time:end:0588c0b1:start=1552418373855730946,finish=1552418456507943148,duration=82652212202
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:15:42] .................................................................................................... 2600/5449
[01:15:46] .................................................................................................... 2700/5449
[01:15:52] .................................................................................................... 2800/5449
[01:15:56] .................................................................................................... 2900/5449
[01:16:00] .....................................F.............................................................. 3000/5449
[01:16:08] .................................................................................................... 3200/5449
[01:16:12] ..........................................................................i......................... 3300/5449
[01:16:16] .................................................................................................... 3400/5449
[01:16:20] ................................................ii...i..ii.......................................... 3500/5449
---
[01:17:25] .................................................................................................... 5000/5449
[01:17:28] .................................................................................................... 5100/5449
[01:17:32] .................................................................................................... 5200/5449
[01:17:35] .................................................................................................... 5300/5449
[01:17:38] .............................................F.........................................i............ 5400/5449
[01:17:40] failures:
[01:17:40] 
[01:17:40] ---- [ui] ui/issues/issue-56411.rs stdout ----
[01:17:40] diff of stderr:
[01:17:40] diff of stderr:
[01:17:40] 
[01:17:40] - error[E0255]: the name `issue_56411_aux` is defined multiple times
[01:17:40] -   --> $DIR/issue-56411.rs:7:21
[01:17:40] + error[E0583]: file not found for module `issue_56411_aux`
[01:17:40] 3    |
[01:17:40] 3    |
[01:17:40] - LL |             mod $name;
[01:17:40] -    |             ---------- previous definition of the module `issue_56411_aux` here
[01:17:40] - LL |             pub use self::$name;
[01:17:40] -    |                     |
[01:17:40] -    |                     |
[01:17:40] -    |                     `issue_56411_aux` reimported here
[01:17:40] - ...
[01:17:40] - ...
[01:17:40] 12 LL | import!(issue_56411_aux);
[01:17:40] -    | ------------------------- in this macro invocation
[01:17:40] 14    |
[01:17:40] 14    |
[01:17:40] -    = note: `issue_56411_aux` must be defined only once in the type namespace of this module
[01:17:40] +    = help: name the file either issue_56411_aux.rs or issue_56411_aux/mod.rs inside the directory "$DIR"
[01:17:40] 16 
[01:17:40] - error[E0365]: `issue_56411_aux` is private, and cannot be re-exported
[01:17:40] -    |
[01:17:40] -    |
[01:17:40] - LL |             pub use self::$name;
[01:17:40] -    |                     ^^^^^^^^^^^ re-export of private `issue_56411_aux`
[01:17:40] - ...
[01:17:40] - LL | import!(issue_56411_aux);
[01:17:40] -    | ------------------------- in this macro invocation
[01:17:40] -    |
[01:17:40] -    = note: consider declaring type or module `issue_56411_aux` with `pub`
[01:17:40] + error: aborting due to previous error
[01:17:40] - error: aborting due to 2 previous errors
[01:17:40] - 
[01:17:40] - Some errors occurred: E0255, E0365.
[01:17:40] - For more information about an error, try `rustc --explain E0255`.
[01:17:40] - For more information about an error, try `rustc --explain E0255`.
[01:17:40] + For more information about this error, try `rustc --explain E0583`.
[01:17:40] 32 
[01:17:40] 
[01:17:40] 
[01:17:40] The actual stderr differed from the expected stderr.
[01:17:40] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/issue-56411.stderr
[01:17:40] To update references, rerun the tests and pass the `--bless` flag
[01:17:40] To only update this specific test, also pass `--test-args issues/issue-56411.rs`
[01:17:40] error: 1 errors occurred comparing output.
[01:17:40] status: exit code: 1
[01:17:40] status: exit code: 1
[01:17:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-56411.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/auxiliary" "-A" "unused"
[01:17:40] ------------------------------------------
[01:17:40] 
[01:17:40] ------------------------------------------
[01:17:40] stderr:
[01:17:40] stderr:
[01:17:40] ------------------------------------------
[01:17:40] {"message":"file not found for module `issue_56411_aux`","code":{"code":"E0583","explanation":"\nA file wasn't found for an out-of-line module.\n\nErroneous code example:\n\n