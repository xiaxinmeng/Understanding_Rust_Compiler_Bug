plain
travis_time:end:09933396:start=1549816281413252028,finish=1549816473890304338,duration=192477052310
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
###############################################################           88.7%
######################################################################## 100.0%
[00:02:13] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:14]     Updating crates.io index
[00:02:24] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:24] Build completed unsuccessfully in 0:00:36
[00:02:24] make: *** [prepare] Error 1
[00:02:24] Makefile:70: recipe for target 'prepare' failed
[00:02:25] Command failed. Attempt 2/5:
[00:02:25] Command failed. Attempt 2/5:
[00:02:25]     Updating crates.io index
[00:02:25] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:25] Build completed unsuccessfully in 0:00:00
[00:02:25] Makefile:70: recipe for target 'prepare' failed
[00:02:25] make: *** [prepare] Error 1
[00:02:27] Command failed. Attempt 3/5:
[00:02:27] Command failed. Attempt 3/5:
[00:02:28]     Updating crates.io index
[00:02:28] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:28] Build completed unsuccessfully in 0:00:00
[00:02:28] Makefile:70: recipe for target 'prepare' failed
[00:02:28] make: *** [prepare] Error 1
[00:02:31] Command failed. Attempt 4/5:
[00:02:31] Command failed. Attempt 4/5:
[00:02:31]     Updating crates.io index
[00:02:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:32] Build completed unsuccessfully in 0:00:00
[00:02:32] make: *** [prepare] Error 1
[00:02:32] Makefile:70: recipe for target 'prepare' failed
[00:02:36] Command failed. Attempt 5/5:
[00:02:36] Command failed. Attempt 5/5:
[00:02:36]     Updating crates.io index
[00:02:36] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:36] Build completed unsuccessfully in 0:00:00
[00:02:36] make: *** [prepare] Error 1
[00:02:36] Makefile:70: recipe for target 'prepare' failed
[00:02:36] The command has failed after 5 attempts.
