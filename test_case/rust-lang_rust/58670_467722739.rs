plain
travis_time:end:05973ba0:start=1551242581652494535,finish=1551242702335874422,duration=120683379887
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
##############################################################            86.3%
######################################################################## 100.0%
[00:01:24] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:24]     Updating crates.io index
[00:01:36] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:36] Build completed unsuccessfully in 0:00:25
[00:01:36] Makefile:70: recipe for target 'prepare' failed
[00:01:36] make: *** [prepare] Error 1
[00:01:37] Command failed. Attempt 2/5:
[00:01:37] Command failed. Attempt 2/5:
[00:01:37] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:37] Build completed unsuccessfully in 0:00:00
[00:01:37] Makefile:70: recipe for target 'prepare' failed
[00:01:37] make: *** [prepare] Error 1
[00:01:39] Command failed. Attempt 3/5:
[00:01:39] Command failed. Attempt 3/5:
[00:01:40] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:40] Build completed unsuccessfully in 0:00:00
[00:01:40] make: *** [prepare] Error 1
[00:01:40] Makefile:70: recipe for target 'prepare' failed
[00:01:43] Command failed. Attempt 4/5:
[00:01:43] Command failed. Attempt 4/5:
[00:01:43] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:43] Build completed unsuccessfully in 0:00:00
[00:01:43] make: *** [prepare] Error 1
[00:01:43] Makefile:70: recipe for target 'prepare' failed
[00:01:47] Command failed. Attempt 5/5:
[00:01:47] Command failed. Attempt 5/5:
[00:01:48] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:48] Build completed unsuccessfully in 0:00:00
[00:01:48] Makefile:70: recipe for target 'prepare' failed
[00:01:48] make: *** [prepare] Error 1
[00:01:48] The command has failed after 5 attempts.
