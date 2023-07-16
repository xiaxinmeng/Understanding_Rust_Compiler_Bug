plain
travis_time:end:007b6c09:start=1554157244701541953,finish=1554157328584379550,duration=83882837597
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
###################################                                       49.5%
######################################################################## 100.0%
[00:02:52] extracting /checkout/obj/build/cache/2019-02-28/cargo-0.34.0-x86_64-unknown-linux-gnu.tar.gz
[00:02:52]     Updating crates.io index
[00:03:06] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:06] Build completed unsuccessfully in 0:00:28
[00:03:06] make: *** [prepare] Error 1
[00:03:06] Makefile:70: recipe for target 'prepare' failed
[00:03:07] Command failed. Attempt 2/5:
[00:03:07] Command failed. Attempt 2/5:
[00:03:07]     Updating crates.io index
[00:03:08] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:08] Build completed unsuccessfully in 0:00:00
[00:03:08] Makefile:70: recipe for target 'prepare' failed
[00:03:08] make: *** [prepare] Error 1
[00:03:10] Command failed. Attempt 3/5:
[00:03:10] Command failed. Attempt 3/5:
[00:03:10]     Updating crates.io index
[00:03:10] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:10] Build completed unsuccessfully in 0:00:00
[00:03:10] make: *** [prepare] Error 1
[00:03:10] Makefile:70: recipe for target 'prepare' failed
[00:03:13] Command failed. Attempt 4/5:
[00:03:13] Command failed. Attempt 4/5:
[00:03:14]     Updating crates.io index
[00:03:14] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:14] Build completed unsuccessfully in 0:00:00
[00:03:14] make: *** [prepare] Error 1
[00:03:14] Makefile:70: recipe for target 'prepare' failed
[00:03:18] Command failed. Attempt 5/5:
[00:03:18] Command failed. Attempt 5/5:
[00:03:18]     Updating crates.io index
[00:03:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:19] Build completed unsuccessfully in 0:00:00
[00:03:19] Makefile:70: recipe for target 'prepare' failed
[00:03:19] make: *** [prepare] Error 1
[00:03:19] The command has failed after 5 attempts.
