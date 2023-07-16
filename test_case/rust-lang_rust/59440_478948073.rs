plain
travis_time:end:04c7a970:start=1554197875312853960,finish=1554197878016263511,duration=2703409551
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:05:22] 
######################################################################## 100.0%
[00:05:23] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:05:23]     Updating crates.io index
[00:05:44]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:05:45]   Downloaded toml v0.4.10
[00:05:45]   Downloaded filetime v0.2.4
[00:05:45]   Downloaded petgraph v0.4.13
[00:05:45]   Downloaded serde v1.0.82
---
tidy check
[00:07:36] * 569 error codes
[00:07:36] * highest error code: E0725
[00:07:36] * 252 features
[00:07:37] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"

[00:07:37] travis_time:end:tidy:start=1554198343462333037,finish=1554198345481226244,duration=2018893207

[00:07:37] Build completed successfully in 0:00:44
---
[00:08:34]    Compiling libc v0.2.51
[00:08:34]    Compiling termcolor v1.0.4
[00:08:34]    Compiling getopts v0.2.17
[00:08:34]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:08:36]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[00:08:46]     Finished release [optimized] target(s) in 12.17s
[00:08:46] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:08:46] travis_fold:end:stage0-test

---
[00:31:26]    Compiling libc v0.2.51
[00:31:26]    Compiling termcolor v1.0.4
[00:31:26]    Compiling getopts v0.2.17
[00:31:26]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:31:29]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[00:31:40]     Finished release [optimized] target(s) in 15.08s
[00:31:40] Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:31:40] travis_fold:end:stage1-test

---
[01:09:03]     Checking termcolor v1.0.4
[01:09:03]     Checking getopts v0.2.17
[01:09:03]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:09:03]     Checking libc v0.2.51
[01:09:04]     Checking libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[01:09:09]     Finished release [optimized] target(s) in 6.86s
[01:09:09] Documenting stage2 whitelisted compiler (x86_64-unknown-linux-gnu)
[01:09:10]  Documenting proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:09:13]     Finished release [optimized] target(s) in 4.13s
---
tidy check
[01:11:59] * 569 error codes
[01:11:59] * highest error code: E0725
[01:11:59] * 252 features
[01:12:00] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"

[01:12:00] travis_time:end:tidy:start=1554202206385614846,finish=1554202208637138248,duration=2251523402

[01:12:00] travis_fold:start:stage0-std
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:54] 
[01:24:54] running 9 tests
[01:24:54] iiiiiiiii
[01:24:54] 
[01:24:54]  finished in 0.165
[01:24:54] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:12] 
[01:25:12] running 121 tests
[01:25:41] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:25:46] i.i......iii.i.....ii
[01:25:46] 
[01:25:46]  finished in 34.897
[01:25:46] travis_fold:end:test_debuginfo

---
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:46] 
[01:25:46] running 20 tests
[01:25:54] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:25:54] ...F................
[01:25:54] 
[01:25:54] ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
[01:25:54] diff of stderr:
[01:25:54] 
[01:25:54] 
[01:25:54] 2    |
[01:25:54] 3    = note: consider adding a `main` function to `$DIR/hash-stable-is-unstable.rs`
[01:25:54] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] + error[E0658]: use of unstable library feature 'rustc_private': crate "rustc_data_structures" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] 7    |
[01:25:54] 8 LL | extern crate rustc_data_structures;
[01:25:54] 
[01:25:54] 10    |
[01:25:54] 10    |
[01:25:54] 11    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:25:54] 12 
[01:25:54] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] + error[E0658]: use of unstable library feature 'rustc_private': crate "rustc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] 15    |
[01:25:54] 16 LL | extern crate rustc;
[01:25:54] 
[01:25:54] 18    |
[01:25:54] 18    |
[01:25:54] 19    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:25:54] 20 
[01:25:54] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] + error[E0658]: use of unstable library feature 'rustc_private': crate "rustc_macros" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] 23    |
[01:25:54] 24 LL | extern crate rustc_macros;
[01:25:54] 
[01:25:54] 26    |
[01:25:54] 26    |
[01:25:54] 27    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:25:54] 28 
[01:25:54] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] + error[E0658]: use of unstable library feature 'rustc_private': crate "rustc_macros" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] 31    |
[01:25:54] 32 LL | use rustc_macros::HashStable;
[01:25:54] 
[01:25:54] 34    |
[01:25:54] 34    |
[01:25:54] 35    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:25:54] 36 
[01:25:54] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] + error[E0658]: use of unstable library feature 'rustc_private': crate "rustc_data_structures" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] 39    |
[01:25:54] 39    |
[01:25:54] 40 LL | #[derive(HashStable)]
[01:25:54] 42    |
[01:25:54] 43    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:25:54] 44 
[01:25:54] - error: aborting due to 6 previous errors
[01:25:54] - error: aborting due to 6 previous errors
[01:25:54] + error[E0658]: use of unstable library feature 'rustc_private': crate "rustc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[01:25:54] +    |
[01:25:54] +    |
[01:25:54] + LL | #[derive(HashStable)]
[01:25:54] +    |
[01:25:54] +    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:25:54] + 
[01:25:54] + error: aborting due to 7 previous errors
[01:25:54] + error: aborting due to 7 previous errors
[01:25:54] 46 
[01:25:54] 47 Some errors occurred: E0601, E0658.
[01:25:54] 48 For more information about an error, try `rustc --explain E0601`.
[01:25:54] 
[01:25:54] 
[01:25:54] The actual stderr differed from the expected stderr.
[01:25:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
[01:25:54] To update references, rerun the tests and pass the `--bless` flag
[01:25:54] To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
[01:25:54] error: 1 errors occurred comparing output.
[01:25:54] status: exit code: 1
[01:25:54] status: exit code: 1
[01:25:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
[01:25:54] ------------------------------------------
[01:25:54] 
[01:25:54] ------------------------------------------
[01:25:54] stderr:
[01:25:54] stderr:
[01:25:54] ------------------------------------------
[01:25:54] {"message":"`main` function not found in crate `hash_stable_is_unstable`","code":{"code":"E0601","explanation":"\nNo `main` function was found in a binary crate. To fix this error, add a\n`main` function. For example:\n\n