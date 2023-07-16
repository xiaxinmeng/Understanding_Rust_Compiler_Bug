plain
travis_time:end:0ddd77e0:start=1545083655294744088,finish=1545083854471210235,duration=199176466147
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:23] Successfully built b660faf4144a
[00:03:23] Successfully tagged rust-ci:latest
[00:03:23] Built container sha256:b660faf4144afe8cc8b2c89388c4a78bf4f7fcd591f5e5a84b5a6924b761a214
[00:03:23] Uploading finished image to s3://rust-lang-ci-sccache2/docker/db9ff5d5ee373ab73faca5f449fef19a4b2c51bffa521b7f96dff69f63bf41d6914752d34f01e887e0176ba22b40cd598769a822432ae334a71fa8d5844e7fa6
[00:04:08] upload failed: - to s3://rust-lang-ci-sccache2/docker/db9ff5d5ee373ab73faca5f449fef19a4b2c51bffa521b7f96dff69f63bf41d6914752d34f01e887e0176ba22b40cd598769a822432ae334a71fa8d5844e7fa6 Unable to locate credentials

[00:04:08] travis_time:end:07809736:start=1545083956659708531,finish=1545084111622661226,duration=154962952695
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:04:08] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
########################################################                  78.2%
######################################################################## 100.0%
[00:04:22] extracting /checkout/obj/build/cache/2018-12-06/cargo-0.32.0-x86_64-unknown-linux-gnu.tar.gz
[00:04:23]     Updating crates.io index
[00:04:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:31] Build completed unsuccessfully in 0:00:22
[00:04:31] make: *** [prepare] Error 1
[00:04:31] Makefile:81: recipe for target 'prepare' failed
[00:04:32] Command failed. Attempt 2/5:
[00:04:32] Command failed. Attempt 2/5:
[00:04:32]     Updating crates.io index
[00:04:32] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:32] Build completed unsuccessfully in 0:00:00
[00:04:32] Makefile:81: recipe for target 'prepare' failed
[00:04:32] make: *** [prepare] Error 1
[00:04:34] Command failed. Attempt 3/5:
[00:04:34] Command failed. Attempt 3/5:
[00:04:34]     Updating crates.io index
[00:04:35] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:35] Build completed unsuccessfully in 0:00:00
[00:04:35] Makefile:81: recipe for target 'prepare' failed
[00:04:35] make: *** [prepare] Error 1
[00:04:38] Command failed. Attempt 4/5:
[00:04:38] Command failed. Attempt 4/5:
[00:04:38]     Updating crates.io index
[00:04:38] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:38] Build completed unsuccessfully in 0:00:00
[00:04:38] Makefile:81: recipe for target 'prepare' failed
[00:04:38] make: *** [prepare] Error 1
