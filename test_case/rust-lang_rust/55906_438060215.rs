plain
travis_time:end:0095aeb8:start=1542063484397050670,finish=1542063545869501563,duration=61472450893
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:02:04] 
################################                                          44.9%
######################################################################## 100.0%
[00:02:04] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:04] error: failed to read `/ena2/Cargo.toml`
[00:02:04] Caused by:
[00:02:04]   No such file or directory (os error 2)
[00:02:04] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:04] Build completed unsuccessfully in 0:00:14
[00:02:04] Build completed unsuccessfully in 0:00:14
[00:02:04] Makefile:81: recipe for target 'prepare' failed
[00:02:04] make: *** [prepare] Error 1
[00:02:05] Command failed. Attempt 2/5:
[00:02:05] error: failed to read `/ena2/Cargo.toml`
[00:02:05] Caused by:
[00:02:05]   No such file or directory (os error 2)
[00:02:05] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:05] Build completed unsuccessfully in 0:00:00
[00:02:05] Build completed unsuccessfully in 0:00:00
[00:02:05] make: *** [prepare] Error 1
[00:02:05] Makefile:81: recipe for target 'prepare' failed
[00:02:07] Command failed. Attempt 3/5:
[00:02:07] error: failed to read `/ena2/Cargo.toml`
[00:02:07] Caused by:
[00:02:07]   No such file or directory (os error 2)
[00:02:07] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:07] Build completed unsuccessfully in 0:00:00
[00:02:07] Build completed unsuccessfully in 0:00:00
[00:02:07] Makefile:81: recipe for target 'prepare' failed
[00:02:07] make: *** [prepare] Error 1
[00:02:10] Command failed. Attempt 4/5:
[00:02:10] error: failed to read `/ena2/Cargo.toml`
[00:02:10] Caused by:
[00:02:10]   No such file or directory (os error 2)
[00:02:10] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:10] Build completed unsuccessfully in 0:00:00
