plain
travis_time:end:2bb09006:start=1549044255499071791,finish=1549044257804178192,duration=2305106401
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
#########################################                                 57.0%
######################################################################## 100.0%
[00:03:14] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:03:14]     Updating crates.io index
[00:03:30] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:30] Build completed unsuccessfully in 0:00:33
[00:03:30] make: *** [prepare] Error 1
[00:03:30] Makefile:70: recipe for target 'prepare' failed
[00:03:31] Command failed. Attempt 2/5:
[00:03:31] Command failed. Attempt 2/5:
[00:03:32] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:32] Build completed unsuccessfully in 0:00:00
[00:03:32] Makefile:70: recipe for target 'prepare' failed
[00:03:32] make: *** [prepare] Error 1
[00:03:34] Command failed. Attempt 3/5:
[00:03:34] Command failed. Attempt 3/5:
[00:03:34] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:34] Build completed unsuccessfully in 0:00:00
[00:03:34] Makefile:70: recipe for target 'prepare' failed
[00:03:34] make: *** [prepare] Error 1
[00:03:37] Command failed. Attempt 4/5:
[00:03:37] Command failed. Attempt 4/5:
[00:03:37] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:37] Build completed unsuccessfully in 0:00:00
[00:03:37] make: *** [prepare] Error 1
[00:03:37] Makefile:70: recipe for target 'prepare' failed
[00:03:41] Command failed. Attempt 5/5:
[00:03:41] Command failed. Attempt 5/5:
[00:03:42] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:42] Build completed unsuccessfully in 0:00:00
[00:03:42] make: *** [prepare] Error 1
[00:03:42] Makefile:70: recipe for target 'prepare' failed
[00:03:42] The command has failed after 5 attempts.
