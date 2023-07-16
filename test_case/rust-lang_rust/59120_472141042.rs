plain
travis_time:end:2e95471f:start=1552413317894481996,finish=1552413320200390527,duration=2305908531
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:11:44] .................................................................................................... 2600/5449
[01:11:48] .................................................................................................... 2700/5449
[01:11:53] .................................................................................................... 2800/5449
[01:11:57] .................................................................................................... 2900/5449
[01:12:01] .......................................F............................................................ 3000/5449
[01:12:08] .................................................................................................... 3200/5449
[01:12:12] ..........................................................................i......................... 3300/5449
[01:12:15] .................................................................................................... 3400/5449
[01:12:19] ................................................ii...i..ii.......................................... 3500/5449
---
[01:13:19] .................................................................................................... 5000/5449
[01:13:22] .................................................................................................... 5100/5449
[01:13:26] .................................................................................................... 5200/5449
[01:13:28] .................................................................................................... 5300/5449
[01:13:31] .............................................F.........................................i............ 5400/5449
[01:13:33] failures:
[01:13:33] 
[01:13:33] ---- [ui] ui/issues/issue-56411.rs stdout ----
[01:13:33] diff of stderr:
[01:13:33] diff of stderr:
[01:13:33] 
[01:13:33] - error[E0255]: the name `issue_56411_aux` is defined multiple times
[01:13:33] -   --> $DIR/issue-56411.rs:7:21
[01:13:33] + error[E0583]: file not found for module `issue_56411_aux`
[01:13:33] 3    |
[01:13:33] 3    |
[01:13:33] - LL |             mod $name;
[01:13:33] -    |             ---------- previous definition of the module `issue_56411_aux` here
[01:13:33] - LL |             pub use self::$name;
[01:13:33] -    |                     |
[01:13:33] -    |                     |
[01:13:33] -    |                     `issue_56411_aux` reimported here
[01:13:33] - ...
[01:13:33] - ...
[01:13:33] 12 LL | import!(issue_56411_aux);
[01:13:33] -    | ------------------------- in this macro invocation
[01:13:33] 14    |
[01:13:33] 14    |
[01:13:33] -    = note: `issue_56411_aux` must be defined only once in the type namespace of this module
[01:13:33] +    = help: name the file either issue_56411_aux.rs or issue_56411_aux/mod.rs inside the directory "$DIR"
[01:13:33] 16 
[01:13:33] - error[E0365]: `issue_56411_aux` is private, and cannot be re-exported
[01:13:33] -    |
[01:13:33] -    |
[01:13:33] - LL |             pub use self::$name;
[01:13:33] -    |                     ^^^^^^^^^^^ re-export of private `issue_56411_aux`
[01:13:33] - ...
[01:13:33] - LL | import!(issue_56411_aux);
[01:13:33] -    | ------------------------- in this macro invocation
[01:13:33] -    |
[01:13:33] -    = note: consider declaring type or module `issue_56411_aux` with `pub`
[01:13:33] + error: aborting due to previous error
[01:13:33] - error: aborting due to 2 previous errors
[01:13:33] - 
[01:13:33] - Some errors occurred: E0255, E0365.
[01:13:33] - For more information about an error, try `rustc --explain E0255`.
[01:13:33] - For more information about an error, try `rustc --explain E0255`.
[01:13:33] + For more information about this error, try `rustc --explain E0583`.
[01:13:33] 32 
[01:13:33] 
[01:13:33] 
[01:13:33] The actual stderr differed from the expected stderr.
[01:13:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/issue-56411.stderr
[01:13:33] To update references, rerun the tests and pass the `--bless` flag
[01:13:33] To only update this specific test, also pass `--test-args issues/issue-56411.rs`
[01:13:33] error: 1 errors occurred comparing output.
[01:13:33] status: exit code: 1
[01:13:33] status: exit code: 1
[01:13:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-56411.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/auxiliary" "-A" "unused"
[01:13:33] ------------------------------------------
[01:13:33] 
[01:13:33] ------------------------------------------
[01:13:33] stderr:
[01:13:33] stderr:
[01:13:33] ------------------------------------------
[01:13:33] {"message":"file not found for module `issue_56411_aux`","code":{"code":"E0583","explanation":"\nA file wasn't found for an out-of-line module.\n\nErroneous code example:\n\n