plain
travis_time:end:0c62b7a0:start=1552422638011955282,finish=1552422640388468460,duration=2376513178
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:07] 
[01:21:07] running 120 tests
[01:21:32] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:21:36] .i......iii.i.....ii
[01:21:36] 
[01:21:36]  finished in 30.003
[01:21:36] travis_fold:end:test_debuginfo

---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:37] 
[01:21:37] running 20 tests
[01:21:43] F.F.................
[01:21:43] 
[01:21:43] ---- [ui] ui-fulldeps/dropck_tarena_unsound_drop.rs stdout ----
[01:21:43] diff of stderr:
[01:21:43] 
[01:21:43] 
[01:21:43] 1 error[E0597]: `arena` does not live long enough
[01:21:43] +   --> $DIR/dropck_tarena_unsound_drop.rs:41:7
[01:21:43] 3    |
[01:21:43] 3    |
[01:21:43] 4 LL |     f(&arena);
[01:21:43] -    |        ^^^^^ borrowed value does not live long enough
[01:21:43] +    |       ^^^^^^ borrowed value does not live long enough
[01:21:43] 6 LL | }
[01:21:43] -    | - `arena` dropped here while still borrowed
[01:21:43] -    |
[01:21:43] -    = note: values in a scope are dropped in the opposite order they are created
[01:21:43] +    | |
[01:21:43] +    | |
[01:21:43] +    | `arena` dropped here while still borrowed
[01:21:43] +    | borrow might be used here, when `arena` is dropped and runs the `Drop` code for type `arena::TypedArena`
[01:21:43] 11 error: aborting due to previous error
[01:21:43] 12 
[01:21:43] 
[01:21:43] 
[01:21:43] 
[01:21:43] The actual stderr differed from the expected stderr.
[01:21:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck_tarena_unsound_drop/dropck_tarena_unsound_drop.stderr
[01:21:43] To update references, rerun the tests and pass the `--bless` flag
[01:21:43] To only update this specific test, also pass `--test-args dropck_tarena_unsound_drop.rs`
[01:21:43] error: 1 errors occurred comparing output.
[01:21:43] status: exit code: 1
[01:21:43] status: exit code: 1
[01:21:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/dropck_tarena_unsound_drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck_tarena_unsound_drop/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck_tarena_unsound_drop/auxiliary" "-A" "unused"
[01:21:43] ------------------------------------------
[01:21:43] 
[01:21:43] ------------------------------------------
[01:21:43] stderr:
[01:21:43] stderr:
[01:21:43] ------------------------------------------
[01:21:43] {"message":"`arena` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a value was dropped while it was still borrowed\n\nExample of erroneous code:\n\n