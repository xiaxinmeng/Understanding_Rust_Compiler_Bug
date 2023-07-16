plain
travis_time:end:011897ee:start=1553701624369405860,finish=1553701758206305691,duration=133836899831
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:12:33] .................................................................................................... 900/5496
[01:12:38] .i...............i.................................................................................. 1000/5496
[01:12:42] .................................iiiii.............................................................. 1100/5496
[01:12:45] .................................................................................................... 1200/5496
[01:12:47] ...........................................F........................................................ 1300/5496
[01:12:53] .................................................................................................... 1500/5496
[01:12:56] .................................................................................................... 1600/5496
[01:13:00] ..........................................i......................................................... 1700/5496
[01:13:03] .................................................................................................... 1800/5496
---
[01:15:25] 
[01:15:25] ---- [ui] ui/error-codes/E0225.rs stdout ----
[01:15:25] diff of stderr:
[01:15:25] 
[01:15:25] 11    |                             -------------- non-auto additional trait
[01:15:25] 12 ...
[01:15:25] 13 LL |     let _: Box<Foo>;
[01:15:25] +    |                ^^^ expanded from this trait alias
[01:15:25] 15 
[01:15:25] 16 error: aborting due to 2 previous errors
[01:15:25] 17 
[01:15:25] 17 
[01:15:25] 
[01:15:25] 
[01:15:25] The actual stderr differed from the expected stderr.
[01:15:25] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/E0225.stderr
[01:15:25] To update references, rerun the tests and pass the `--bless` flag
[01:15:25] To only update this specific test, also pass `--test-args error-codes/E0225.rs`
[01:15:25] error: 1 errors occurred comparing output.
[01:15:25] status: exit code: 1
[01:15:25] status: exit code: 1
[01:15:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0225.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/auxiliary" "-A" "unused"
[01:15:25] ------------------------------------------
[01:15:25] 
[01:15:25] ------------------------------------------
[01:15:25] stderr:
[01:15:25] stderr:
[01:15:25] ------------------------------------------
[01:15:25] {"message":"only auto traits can be used as additional traits in a trait object","code":{"code":"E0225","explanation":"\nYou attempted to use multiple types as bounds for a closure or trait object.\nRust does not currently support this. A simple example that causes this error:\n\n