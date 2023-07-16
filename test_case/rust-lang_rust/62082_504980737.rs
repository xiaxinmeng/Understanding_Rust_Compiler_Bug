plain
travis_time:end:25185900:start=1561377658942348717,finish=1561377748990608061,duration=90048259344
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:01:09] 
######################################################################## 100.0%
[00:01:09] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:09]     Updating crates.io index
[00:01:29] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:29] Build completed unsuccessfully in 0:00:33
[00:01:29] make: *** [prepare] Error 1
[00:01:29] Makefile:69: recipe for target 'prepare' failed
[00:01:30] Command failed. Attempt 2/5:
[00:01:30] Command failed. Attempt 2/5:
[00:01:30]     Updating crates.io index
[00:01:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] make: *** [prepare] Error 1
[00:01:31] Makefile:69: recipe for target 'prepare' failed
[00:01:33] Command failed. Attempt 3/5:
[00:01:33] Command failed. Attempt 3/5:
[00:01:33]     Updating crates.io index
[00:01:33] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:33] Build completed unsuccessfully in 0:00:00
[00:01:33] Makefile:69: recipe for target 'prepare' failed
[00:01:33] make: *** [prepare] Error 1
[00:01:36] Command failed. Attempt 4/5:
[00:01:36] Command failed. Attempt 4/5:
[00:01:36]     Updating crates.io index
[00:01:37] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:37] Build completed unsuccessfully in 0:00:00
[00:01:37] make: *** [prepare] Error 1
[00:01:37] Makefile:69: recipe for target 'prepare' failed
[00:01:41] Command failed. Attempt 5/5:
[00:01:41] Command failed. Attempt 5/5:
[00:01:41]     Updating crates.io index
[00:01:41] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:41] make: *** [prepare] Error 1
[00:01:41] Makefile:69: recipe for target 'prepare' failed
[00:01:41] The command has failed after 5 attempts.
