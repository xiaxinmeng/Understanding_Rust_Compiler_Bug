plain
travis_time:end:1c32eb34:start=1549062836042676408,finish=1549062839256178723,duration=3213502315
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:04:21] .................................................................................................... 2500/5361
[01:04:25] .................................................................................................... 2600/5361
[01:04:29] .................................................................................................... 2700/5361
[01:04:34] .................................................................................................... 2800/5361
[01:04:38] ..............F..................................................................................... 2900/5361
[01:04:45] .................................................................................................... 3100/5361
[01:04:50] .................................................................................................... 3200/5361
[01:04:54] ...................i................................................................................ 3300/5361
[01:04:57] ....................................................................................ii...i..ii...... 3400/5361
---
[01:05:44] ............i...................................................................................F... 4600/5361
[01:05:50] .................................................................................................... 4700/5361
[01:05:53] .................................................................................................... 4800/5361
[01:05:58] .................................................................................................... 4900/5361
[01:06:01] ...................................................................................................F 5000/5361
[01:06:05] ...................................F................................................................ 5100/5361
[01:06:11] .................................................................................................... 5300/5361
[01:06:11] .................................................................................................... 5300/5361
[01:06:13] i...........................................F................
[01:06:13] 
[01:06:13] ---- [ui] ui/feature-gates/feature-gate-generic_associated_types.rs stdout ----
[01:06:13] diff of stderr:
[01:06:13] 
[01:06:13] 
[01:06:13] 14    |
[01:06:13] 15    = help: add #![feature(generic_associated_types)] to the crate attributes to enable
[01:06:13] 16 
[01:06:13] - error[E0658]: where clauses on associated types are unstable (see issue #44265)
[01:06:13] + error[E0658]: where-clauses on associated types are unstable (see issue #44265)
[01:06:13] 18   --> $DIR/feature-gate-generic_associated_types.rs:6:5
[01:06:13] 19    |
[01:06:13] 20 LL |     type Pointer2<T>: Deref<Target = T> where T: Clone, U: Clone;
[01:06:13] 38    |
[01:06:13] 39    = help: add #![feature(generic_associated_types)] to the crate attributes to enable
[01:06:13] 40 
[01:06:13] - error[E0658]: where clauses on associated types are unstable (see issue #44265)
[01:06:13] - error[E0658]: where clauses on associated types are unstable (see issue #44265)
[01:06:13] + error[E0658]: where-clauses on associated types are unstable (see issue #44265)
[01:06:13] 42   --> $DIR/feature-gate-generic_associated_types.rs:21:5
[01:06:13] 43    |
[01:06:13] 44 LL |     type Assoc where Self: Sized;
[01:06:13] 46    |
[01:06:13] 47    = help: add #![feature(generic_associated_types)] to the crate attributes to enable
[01:06:13] 48 
[01:06:13] - error[E0658]: where clauses on associated types are unstable (see issue #44265)
[01:06:13] - error[E0658]: where clauses on associated types are unstable (see issue #44265)
[01:06:13] + error[E0658]: where-clauses on associated types are unstable (see issue #44265)
[01:06:13] 50   --> $DIR/feature-gate-generic_associated_types.rs:26:5
[01:06:13] 51    |
[01:06:13] 52 LL |     type Assoc where Self: Sized = Foo;
[01:06:13] 
[01:06:13] The actual stderr differed from the expected stderr.
[01:06:13] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-generic_associated_types/feature-gate-generic_associated_types.stderr
[01:06:13] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-generic_associated_types/feature-gate-generic_associated_types.stderr
[01:06:13] To update references, rerun the tests and pass the `--bless` flag
[01:06:13] To only update this specific test, also pass `--test-args feature-gates/feature-gate-generic_associated_types.rs`
[01:06:13] error: 1 errors occurred comparing output.
[01:06:13] status: exit code: 1
[01:06:13] status: exit code: 1
[01:06:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-generic_associated_types/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-generic_associated_types/auxiliary" "-A" "unused"
[01:06:13] ------------------------------------------
[01:06:13] 
[01:06:13] ------------------------------------------
[01:06:13] stderr:
[01:06:13] stderr:
[01:06:13] ------------------------------------------
[01:06:13] {"message":"generic associated types are unstable (see issue #44265)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n