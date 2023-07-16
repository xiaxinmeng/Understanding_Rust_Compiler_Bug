plain
travis_time:end:05f5fc9e:start=1554939708519251746,finish=1554939801770584426,duration=93251332680
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
[01:18:10] 
[01:18:10] running 9 tests
[01:18:10] iiiiiiiii
[01:18:10] 
[01:18:10]  finished in 0.153
[01:18:10] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:26] 
[01:18:26] running 121 tests
[01:18:52] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:18:57] i.i......iii.i.....ii
[01:18:57] 
[01:18:57]  finished in 30.757
[01:18:57] travis_fold:end:test_debuginfo

---
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:57] 
[01:18:57] running 22 tests
[01:19:05] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:19:05] ....F.F...............
[01:19:05] 
[01:19:05] ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
[01:19:05] diff of stderr:
[01:19:05] 
[01:19:05] 
[01:19:05] 2    |
[01:19:05] 3    = note: consider adding a `main` function to `$DIR/hash-stable-is-unstable.rs`
[01:19:05] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:19:05] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
[01:19:05] 6   --> $DIR/hash-stable-is-unstable.rs:3:1
[01:19:05] 7    |
---
[01:19:05] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:19:05] + error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
[01:19:05] 38   --> $DIR/hash-stable-is-unstable.rs:13:10
[01:19:05] 39    |
[01:19:05] 40 LL | #[derive(HashStable)]
[01:19:05] 41    |          ^^^^^^^^^^
[01:19:05] 42    |
[01:19:05] +    = note: for more information, see tracking issue #27812
[01:19:05] 43    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:19:05] 43    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:19:05] 44 
[01:19:05] 45 error: aborting due to 6 previous errors
[01:19:05] 
[01:19:05] 
[01:19:05] The actual stderr differed from the expected stderr.
[01:19:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
[01:19:05] To update references, rerun the tests and pass the `--bless` flag
[01:19:05] To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
[01:19:05] error: 1 errors occurred comparing output.
[01:19:05] status: exit code: 1
[01:19:05] status: exit code: 1
[01:19:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
[01:19:05] ------------------------------------------
[01:19:05] 
[01:19:05] ------------------------------------------
[01:19:05] stderr:
[01:19:05] stderr:
[01:19:05] ------------------------------------------
[01:19:05] {"message":"`main` function not found in crate `hash_stable_is_unstable`","code":{"code":"E0601","explanation":"\nNo `main` function was found in a binary crate. To fix this error, add a\n`main` function. For example:\n\n