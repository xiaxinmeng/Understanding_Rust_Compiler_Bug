plain
travis_time:end:0051b622:start=1559577912057949456,finish=1559577912879654199,duration=821704743
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:02:05] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:05] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:05] 
[00:02:05] Caused by:
[00:02:05]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:05] Build completed unsuccessfully in 0:00:23
[00:02:05] make: *** [prepare] Error 1
[00:02:05] Makefile:69: recipe for target 'prepare' failed
[00:02:06] Command failed. Attempt 2/5:
[00:02:06] Command failed. Attempt 2/5:
[00:02:07] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:07] 
[00:02:07] Caused by:
[00:02:07]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:07] Build completed unsuccessfully in 0:00:00
[00:02:07] make: *** [prepare] Error 1
[00:02:07] Makefile:69: recipe for target 'prepare' failed
[00:02:09] Command failed. Attempt 3/5:
[00:02:09] Command failed. Attempt 3/5:
[00:02:09] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:09] 
[00:02:09] Caused by:
[00:02:09]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:09] Build completed unsuccessfully in 0:00:00
[00:02:09] make: *** [prepare] Error 1
[00:02:09] Makefile:69: recipe for target 'prepare' failed
[00:02:12] Command failed. Attempt 4/5:
[00:02:12] Command failed. Attempt 4/5:
[00:02:12] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:12] 
[00:02:12] Caused by:
[00:02:12]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:12] Build completed unsuccessfully in 0:00:00
[00:02:12] Makefile:69: recipe for target 'prepare' failed
[00:02:12] make: *** [prepare] Error 1
[00:02:16] Command failed. Attempt 5/5:
[00:02:16] Command failed. Attempt 5/5:
[00:02:16] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:16] 
[00:02:16] Caused by:
[00:02:16]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:16] Build completed unsuccessfully in 0:00:00
[00:02:16] make: *** [prepare] Error 1
[00:02:16] Makefile:69: recipe for target 'prepare' failed
[00:02:16] The command has failed after 5 attempts.
