plain
travis_time:end:01af613e:start=1542840983454530064,finish=1542840986009267361,duration=2554737297
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
######################################################################    97.8%
######################################################################## 100.0%
[00:02:11] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:11]     Updating crates.io index
[00:02:18] error: the lock file /checkout/src/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:18] Build completed unsuccessfully in 0:00:23
[00:02:18] Makefile:81: recipe for target 'prepare' failed
[00:02:18] make: *** [prepare] Error 1
[00:02:19] Command failed. Attempt 2/5:
[00:02:19] Command failed. Attempt 2/5:
[00:02:19]     Updating crates.io index
[00:02:19] error: the lock file /checkout/src/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:19] Build completed unsuccessfully in 0:00:00
[00:02:19] make: *** [prepare] Error 1
[00:02:19] Makefile:81: recipe for target 'prepare' failed
[00:02:21] Command failed. Attempt 3/5:
[00:02:21] Command failed. Attempt 3/5:
[00:02:21]     Updating crates.io index
[00:02:22] error: the lock file /checkout/src/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:22] Build completed unsuccessfully in 0:00:00
[00:02:22] make: *** [prepare] Error 1
[00:02:22] Makefile:81: recipe for target 'prepare' failed
[00:02:25] Command failed. Attempt 4/5:
[00:02:25] Command failed. Attempt 4/5:
[00:02:25]     Updating crates.io index
[00:02:25] error: the lock file /checkout/src/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:25] Build completed unsuccessfully in 0:00:00
[00:02:25] make: *** [prepare] Error 1
[00:02:25] Makefile:81: recipe for target 'prepare' failed
[00:02:25] Makefile:81: recipe for target 'prepare' failed
[00:02:30] error: the lock file /checkout/src/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:30] Build completed unsuccessfully in 0:00:00
[00:02:30] Makefile:81: recipe for target 'prepare' failed
[00:02:30] make: *** [prepare] Error 1
[00:02:30] The command has failed after 5 attempts.
