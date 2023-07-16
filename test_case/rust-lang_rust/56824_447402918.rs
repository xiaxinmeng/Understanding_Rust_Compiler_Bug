plain
travis_time:end:00cf2c79:start=1544807335307619232,finish=1544807338304616753,duration=2996997521
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:45:59] .................................................................................................... 2100/5169
[00:46:04] .................................................................................................... 2200/5169
[00:46:07] .................................................................................................... 2300/5169
[00:46:11] .................................................................................................... 2400/5169
[00:46:15] ........................................................................................F........... 2500/5169
[00:46:22] .................................................................................................... 2700/5169
[00:46:26] .................................................................................................... 2800/5169
[00:46:29] .................................................................................................... 2900/5169
[00:46:32] .................................................................................................... 3000/5169
---
[00:47:46] 
[00:47:46] ---- [ui] ui/issues/issue-37887.rs stdout ----
[00:47:46] diff of stderr:
[00:47:46] 
[00:47:46] 4 LL |     use libc::*; //~ ERROR unresolved import
[00:47:46] 5    |         ^^^^ maybe a missing `extern crate libc;`?
[00:47:46] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[00:47:46] - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
[00:47:46] + error[E0658]: use of unstable library feature 'rustc_private': This crate is being loaded from the sysroot, a permanently unstable location for private compiler dependencies. It is not intended for general use. Prefer using a public version of this crate from [crates.io](https://crates.io) via [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html). (see issue #27812)
[00:47:46] 9    |
[00:47:46] 9    |
[00:47:46] 10 LL |     extern crate libc; //~ ERROR use of unstable
[00:47:46] 
[00:47:46] The actual stderr differed from the expected stderr.
[00:47:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37887/issue-37887.stderr
[00:47:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37887/issue-37887.stderr
[00:47:46] To update references, rerun the tests and pass the `--bless` flag
[00:47:46] To only update this specific test, also pass `--test-args issues/issue-37887.rs`
[00:47:46] error: 1 errors occurred comparing output.
[00:47:46] status: exit code: 1
[00:47:46] status: exit code: 1
[00:47:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37887.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37887/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37887/auxiliary" "-A" "unused"
[00:47:46] ------------------------------------------
[00:47:46] 
[00:47:46] ------------------------------------------
[00:47:46] stderr:
[00:47:46] stderr:
[00:47:46] ------------------------------------------
[00:47:46] {"message":"unresolved import `libc`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n