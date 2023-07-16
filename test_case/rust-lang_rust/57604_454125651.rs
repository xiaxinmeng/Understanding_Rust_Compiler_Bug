plain
travis_time:end:2b8f2c3c:start=1547489114052355413,finish=1547489252298054981,duration=138245699568
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:05:37] .................................................................................................... 4400/5303
[01:05:40] ..............................................................i..................................... 4500/5303
[01:05:47] .................................................................................................... 4600/5303
[01:05:50] .................................................................................................... 4700/5303
[01:05:53] .................................................................................F....F............. 4800/5303
[01:06:01] .................................................................................................... 5000/5303
[01:06:04] .................................................................................................... 5100/5303
[01:06:07] .................................................................................................... 5200/5303
[01:06:10] ..........................................i......................................................... 5300/5303
[01:06:10] ..........................................i......................................................... 5300/5303
[01:06:10] ...
[01:06:10] failures:
[01:06:10] 
[01:06:10] ---- [ui] ui/str/str-idx.rs stdout ----
[01:06:10] diff of stderr:
[01:06:10] 
[01:06:10] - error[E0277]: the type `str` cannot be indexed by `{integer}`
[01:06:10] + error[E0277]: the trait bound `{integer}: std::slice::SliceIndex<str>` is not satisfied
[01:06:10] 3    |
[01:06:10] 3    |
[01:06:10] 4 LL |     let c: u8 = s[4]; //~ ERROR the type `str` cannot be indexed by `{integer}`
[01:06:10] -    |                 ^^^^ `str` cannot be indexed by `{integer}`
[01:06:10] -    |                 ^^^^ `str` cannot be indexed by `{integer}`
[01:06:10] +    |                 ^^^^ slice indices are of type `usize` or ranges of `usize`
[01:06:10] 6    |
[01:06:10] -    = help: the trait `std::ops::Index<{integer}>` is not implemented for `str`
[01:06:10] +    = help: the trait `std::slice::SliceIndex<str>` is not implemented for `{integer}`
[01:06:10] +    = note: required because of the requirements on the impl of `std::ops::Index<{integer}>` for `str`
[01:06:10] 9 error: aborting due to previous error
[01:06:10] 10 
[01:06:10] 
[01:06:10] 
[01:06:10] 
[01:06:10] The actual stderr differed from the expected stderr.
[01:06:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx/str-idx.stderr
[01:06:10] To update references, rerun the tests and pass the `--bless` flag
[01:06:10] To only update this specific test, also pass `--test-args str/str-idx.rs`
[01:06:10] error: 1 errors occurred comparing output.
[01:06:10] status: exit code: 1
[01:06:10] status: exit code: 1
[01:06:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/str/str-idx.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx/auxiliary" "-A" "unused"
[01:06:10] ------------------------------------------
[01:06:10] 
[01:06:10] ------------------------------------------
[01:06:10] stderr:
[01:06:10] stderr:
[01:06:10] ------------------------------------------
[01:06:10] {"message":"the trait bound `{integer}: std::slice::SliceIndex<str>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n