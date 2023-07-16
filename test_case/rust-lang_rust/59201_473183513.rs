plain
travis_time:end:19b38dc0:start=1552629816587646881,finish=1552629817544235677,duration=956588796
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:14:15] .................................................................................................... 4400/5467
[01:14:18] .................................................................................................... 4500/5467
[01:14:22] .................................................................................................... 4600/5467
[01:14:27] ...........i........................................................................................ 4700/5467
[01:14:32] .................................................................F.................................. 4800/5467
[01:14:40] .................................................................................................... 5000/5467
[01:14:43] .................................................................................................... 5100/5467
[01:14:47] .................................................................................................... 5200/5467
[01:14:50] .................................................................................................... 5300/5467
[01:14:50] .................................................................................................... 5300/5467
[01:14:53] .................................................................................................... 5400/5467
[01:14:55] .....i.............................................................
[01:14:55] failures:
[01:14:55] 
[01:14:55] ---- [ui] ui/simd-type.rs stdout ----
[01:14:55] diff of stderr:
[01:14:55] 
[01:14:55] 10 LL | struct i64f64(i64, f64);
[01:14:55] 11    | ^^^^^^^^^^^^^^^^^^^^^^^^ SIMD elements must have the same type
[01:14:55] 12 
[01:14:55] - error[E0077]: SIMD vector element type should be machine type
[01:14:55] -    |
[01:14:55] -    |
[01:14:55] - LL | struct int4(isize, isize, isize, isize);
[01:14:55] + error: aborting due to 2 previous errors
[01:14:55] 18 
[01:14:55] - error: aborting due to 3 previous errors
[01:14:55] - 
[01:14:55] - 
[01:14:55] - Some errors occurred: E0075, E0076, E0077.
[01:14:55] + Some errors occurred: E0075, E0076.
[01:14:55] 22 For more information about an error, try `rustc --explain E0075`.
[01:14:55] 23 
[01:14:55] 
[01:14:55] 
[01:14:55] The actual stderr differed from the expected stderr.
[01:14:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-type/simd-type.stderr
[01:14:55] To update references, rerun the tests and pass the `--bless` flag
[01:14:55] To only update this specific test, also pass `--test-args simd-type.rs`
[01:14:55] error: 1 errors occurred comparing output.
[01:14:55] status: exit code: 1
[01:14:55] status: exit code: 1
[01:14:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-type/auxiliary" "-A" "unused"
[01:14:55] ------------------------------------------
[01:14:55] 
[01:14:55] ------------------------------------------
[01:14:55] stderr:
[01:14:55] stderr:
[01:14:55] ------------------------------------------
[01:14:55] {"message":"SIMD vector cannot be empty","code":{"code":"E0075","explanation":"\nThe `#[simd]` attribute can only be applied to non empty tuple structs, because\nit doesn't make sense to try to use SIMD operations when there are no values to\noperate on.\n\nThis will cause an error:\n\n