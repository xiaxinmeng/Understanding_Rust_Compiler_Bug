plain
travis_time:end:0d6dfe30:start=1553978227791232532,finish=1553978230171093863,duration=2379861331
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:07:08] ..........................................i......................................................... 600/5513
[01:07:13] .................................................................................................... 700/5513
[01:07:18] .................................................................................................... 800/5513
[01:07:23] .................................................................................................... 900/5513
[01:07:29] .i...........F...i.................................................................................. 1000/5513
[01:07:33] ..................................iiiii............................................................. 1100/5513
[01:07:40] .................................................................................................... 1300/5513
[01:07:43] .................................................................................................... 1400/5513
[01:07:47] .................................................................................................... 1500/5513
[01:07:50] .................................................................................................... 1600/5513
[01:07:50] .................................................................................................... 1600/5513
[01:07:54] ..................................................i................................................. 1700/5513
[01:07:58] .................................................................................................... 1800/5513
[01:08:02] .................................................................................................... 1900/5513
[01:08:06] .................................................................................................... 2000/5513
[01:08:10] ....................................................................................i............... 2100/5513
[01:08:14] .................................................................................................... 2200/5513
[01:08:19] ..................................F.............F................................................... 2300/5513
[01:08:29] .................................................................................................... 2500/5513
[01:08:33] .................................................................................................... 2600/5513
[01:08:37] .................................................................................................... 2700/5513
[01:08:43] .................................................................................................... 2800/5513
---
[01:10:42] 
[01:10:42] ---- [ui] ui/cycle-projection-based-on-where-clause.rs stdout ----
[01:10:42] diff of stderr:
[01:10:42] 
[01:10:42] 11 LL |           T : Add<T::Item>
[01:10:42] 13 
[01:10:42] 13 
[01:10:42] - error[E0220]: associated type `Item` not found for `T`
[01:10:42] -   --> $DIR/cycle-projection-based-on-where-clause.rs:17:19
[01:10:42] -    |
[01:10:42] - LL |           T : Add<T::Item>
[01:10:42] -    |                   ^^^^^^^ associated type `Item` not found
[01:10:42] + error: aborting due to previous error
[01:10:42] - error: aborting due to 2 previous errors
[01:10:42] - 
[01:10:42] - Some errors occurred: E0220, E0391.
[01:10:42] - For more information about an error, try `rustc --explain E0220`.
[01:10:42] - For more information about an error, try `rustc --explain E0220`.
[01:10:42] + For more information about this error, try `rustc --explain E0391`.
[01:10:42] 24 
[01:10:42] 
[01:10:42] 
[01:10:42] The actual stderr differed from the expected stderr.
[01:10:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-projection-based-on-where-clause/cycle-projection-based-on-where-clause.stderr
[01:10:42] To update references, rerun the tests and pass the `--bless` flag
[01:10:42] To only update this specific test, also pass `--test-args cycle-projection-based-on-where-clause.rs`
[01:10:42] error: 1 errors occurred comparing output.
[01:10:42] status: exit code: 1
[01:10:42] status: exit code: 1
[01:10:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-projection-based-on-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-projection-based-on-where-clause/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-projection-based-on-where-clause/auxiliary" "-A" "unused"
[01:10:42] ------------------------------------------
[01:10:42] 
[01:10:42] ------------------------------------------
[01:10:42] stderr:
[01:10:42] stderr:
[01:10:42] ------------------------------------------
[01:10:42] {"message":"cycle detected when computing the bounds for type parameter `T`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n