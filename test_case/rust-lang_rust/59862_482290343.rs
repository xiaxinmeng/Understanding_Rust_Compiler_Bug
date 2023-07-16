plain
travis_time:end:011b3554:start=1555008221155151185,finish=1555008346608386517,duration=125453235332
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:02] 
[01:15:02] running 9 tests
[01:15:02] iiiiiiiii
[01:15:02] 
[01:15:02]  finished in 0.158
[01:15:02] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:19] 
[01:15:19] running 121 tests
[01:15:49] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:15:54] i.i......iii.i.....ii
[01:15:54] 
[01:15:54]  finished in 35.228
[01:15:54] travis_fold:end:test_debuginfo

---
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:54] 
[01:15:54] running 22 tests
[01:16:02] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:16:02] ....F.F...............
[01:16:02] 
[01:16:02] ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
[01:16:02] diff of stderr:
[01:16:02] 
[01:16:02] 
[01:16:02] 2    |
[01:16:02] 3    = note: consider adding a `main` function to `$DIR/hash-stable-is-unstable.rs`
[01:16:02] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:16:02] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
[01:16:02] 6   --> $DIR/hash-stable-is-unstable.rs:3:1
[01:16:02] 7    |
---
[01:16:02] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:16:02] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
[01:16:02] 38   --> $DIR/hash-stable-is-unstable.rs:13:10
[01:16:02] 39    |
[01:16:02] 40 LL | #[derive(HashStable)]
[01:16:02] 41    |          ^^^^^^^^^^
[01:16:02] 42    |
[01:16:02] +    = note: for more information, see https://github.com/rust-lang/rust/issues/27812
[01:16:02] 43    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:16:02] 43    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:16:02] 44 
[01:16:02] 45 error: aborting due to 6 previous errors
[01:16:02] 
[01:16:02] 
[01:16:02] The actual stderr differed from the expected stderr.
[01:16:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
[01:16:02] To update references, rerun the tests and pass the `--bless` flag
[01:16:02] To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
[01:16:02] error: 1 errors occurred comparing output.
[01:16:02] status: exit code: 1
[01:16:02] status: exit code: 1
[01:16:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
[01:16:02] ------------------------------------------
[01:16:02] 
[01:16:02] ------------------------------------------
[01:16:02] stderr:
[01:16:02] stderr:
[01:16:02] ------------------------------------------
[01:16:02] {"message":"`main` function not found in crate `hash_stable_is_unstable`","code":{"code":"E0601","explanation":"\nNo `main` function was found in a binary crate. To fix this error, add a\n`main` function. For example:\n\n