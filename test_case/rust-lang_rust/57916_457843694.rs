plain
travis_time:end:0f873c2f:start=1548515656784813265,finish=1548515735215515988,duration=78430702723
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:57:52] .................................................................................................... 500/5345
[00:57:56] .............................i...................................................................... 600/5345
[00:57:59] .................................................................................................... 700/5345
[00:58:04] .................................................................................................... 800/5345
[00:58:10] .............................................F...........................i...............i.......... 900/5345
[00:58:13] ...................................................................................................i 1000/5345
[00:58:20] .................................................................................................... 1200/5345
[00:58:22] .................................................................................................... 1300/5345
[00:58:25] .................................................................................................... 1400/5345
[00:58:28] .................................................................................................... 1500/5345
[00:58:28] .................................................................................................... 1500/5345
[00:58:31] ........................................F.................................................i......... 1600/5345
[00:58:37] .................................................................................................... 1800/5345
[00:58:41] .................................................................................................... 1900/5345
[00:58:45] .................................................................................................... 2000/5345
[00:58:48] .................i.................................................................................. 2100/5345
---
[01:00:34] .................................................................................................... 4800/5345
[01:00:39] .................................................................................................... 4900/5345
[01:00:42] .................................................................................................... 5000/5345
[01:00:46] .................................................................................................... 5100/5345
[01:00:48] ...........................................................................F.F...................... 5200/5345
[01:00:53] .............................................
[01:00:53] failures:
[01:00:53] 
[01:00:53] ---- [ui] ui/consts/min_const_fn/min_const_fn_unsafe.rs stdout ----
[01:00:53] ---- [ui] ui/consts/min_const_fn/min_const_fn_unsafe.rs stdout ----
[01:00:53] diff of stderr:
[01:00:53] 
[01:00:53] + error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
[01:00:53] +    |
[01:00:53] +    |
[01:00:53] + LL | const fn bad_const_fn_deref_raw(x: *mut usize) -> &'static usize { unsafe { &*x } } //~ is unsafe
[01:00:53] +    |                                                                             ^^^ dereference of raw pointer
[01:00:53] +    |
[01:00:53] +    = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
[01:00:53] + 
[01:00:53] 1 error[E0658]: dereferencing raw pointers in constant functions is unstable (see issue #51911)
[01:00:53] 3    |
[01:00:53] 
[01:00:53] 29    |     ^^^^^^^^^^^^^^^
[01:00:53] 30    |
[01:00:53] 30    |
[01:00:53] 31    = help: add #![feature(const_fn_union)] to the crate attributes to enable
[01:00:53] - 
[01:00:53] - error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
[01:00:53] -    |
[01:00:53] -    |
[01:00:53] - LL | const fn bad_const_fn_deref_raw(x: *mut usize) -> &'static usize { unsafe { &*x } } //~ is unsafe
[01:00:53] -    |                                                                             ^^^ dereference of raw pointer
[01:00:53] -    |
[01:00:53] -    = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
[01:00:53] 41 error: aborting due to 5 previous errors
[01:00:53] 42 
[01:00:53] 
[01:00:53] 
[01:00:53] 
[01:00:53] The actual stderr differed from the expected stderr.
[01:00:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_unsafe/min_const_fn_unsafe.stderr
[01:00:53] To update references, rerun the tests and pass the `--bless` flag
[01:00:53] To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_unsafe.rs`
[01:00:53] error: 1 errors occurred comparing output.
[01:00:53] status: exit code: 1
[01:00:53] status: exit code: 1
[01:00:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_unsafe.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_unsafe/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_unsafe/auxiliary" "-A" "unused"
[01:00:53] ------------------------------------------
[01:00:53] 
[01:00:53] ------------------------------------------
[01:00:53] stderr:
[01:00:53] stderr:
[01:00:53] ------------------------------------------
[01:00:53] {"message":"dereference of raw pointer is unsafe and requires unsafe function or block","code":{"code":"E0133","explanation":"\nUnsafe code was used outside of an unsafe function or block.\n\nErroneous code example:\n\n