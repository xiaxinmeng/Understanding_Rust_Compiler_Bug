plain
travis_time:end:01bf40e0:start=1548179329381587011,finish=1548179451168497109,duration=121786910098
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
############################################################              84.7%
######################################################################    98.0%
######################################################################## 100.0%
[00:01:30] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:30] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:01:30] Caused by:
[00:01:30]   No such file or directory (os error 2)
[00:01:30] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:30] Build completed unsuccessfully in 0:00:14
[00:01:30] Build completed unsuccessfully in 0:00:14
[00:01:30] Makefile:71: recipe for target 'prepare' failed
[00:01:30] make: *** [prepare] Error 1
[00:01:31] Command failed. Attempt 2/5:
[00:01:31] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:01:31] Caused by:
[00:01:31]   No such file or directory (os error 2)
[00:01:31] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] Makefile:71: recipe for target 'prepare' failed
[00:01:31] make: *** [prepare] Error 1
[00:01:33] Command failed. Attempt 3/5:
[00:01:33] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:01:33] Caused by:
[00:01:33]   No such file or directory (os error 2)
[00:01:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:33] Build completed unsuccessfully in 0:00:00
[00:01:33] Build completed unsuccessfully in 0:00:00
[00:01:33] make: *** [prepare] Error 1
[00:01:33] Makefile:71: recipe for target 'prepare' failed
[00:01:36] Command failed. Attempt 4/5:
[00:01:36] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:01:36] Caused by:
[00:01:36]   No such file or directory (os error 2)
[00:01:36] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:36] Build completed unsuccessfully in 0:00:00
[00:01:36] Build completed unsuccessfully in 0:00:00
[00:01:36] make: *** [prepare] Error 1
[00:01:36] Makefile:71: recipe for target 'prepare' failed
[00:01:40] Command failed. Attempt 5/5:
[00:01:40] error: failed to read `/par/rayon-tlv/Cargo.toml`
[00:01:40] Caused by:
[00:01:40]   No such file or directory (os error 2)
[00:01:40] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:40] Build completed unsuccessfully in 0:00:00
