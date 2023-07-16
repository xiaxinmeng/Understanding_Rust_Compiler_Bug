plain
travis_time:end:005b6708:start=1541029362377814034,finish=1541029364477818882,duration=2100004848
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
###################################                                       49.2%
######################################################################## 100.0%
[00:02:26] extracting /checkout/obj/build/cache/2018-10-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:26]     Updating crates.io index
[00:02:31] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:31] Build completed unsuccessfully in 0:00:22
[00:02:31] Makefile:81: recipe for target 'prepare' failed
[00:02:31] make: *** [prepare] Error 1
[00:02:32] Command failed. Attempt 2/5:
[00:02:32] Command failed. Attempt 2/5:
[00:02:32] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:32] Build completed unsuccessfully in 0:00:00
[00:02:32] make: *** [prepare] Error 1
[00:02:32] Makefile:81: recipe for target 'prepare' failed
[00:02:34] Command failed. Attempt 3/5:
[00:02:34] Command failed. Attempt 3/5:
[00:02:35] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:35] Build completed unsuccessfully in 0:00:00
[00:02:35] make: *** [prepare] Error 1
[00:02:35] Makefile:81: recipe for target 'prepare' failed
[00:02:38] Command failed. Attempt 4/5:
[00:02:38] Command failed. Attempt 4/5:
[00:02:38] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:38] Build completed unsuccessfully in 0:00:00
[00:02:38] make: *** [prepare] Error 1
[00:02:38] Makefile:81: recipe for target 'prepare' failed
[00:02:42] Command failed. Attempt 5/5:
[00:02:42] Command failed. Attempt 5/5:
[00:02:42] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:42] Build completed unsuccessfully in 0:00:00
[00:02:42] Makefile:81: recipe for target 'prepare' failed
[00:02:42] make: *** [prepare] Error 1
[00:02:42] The command has failed after 5 attempts.
