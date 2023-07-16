plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:04d740e0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
############################################################              84.6%
######################################################################## 100.0%
[00:04:49] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:04:49]     Updating crates.io index
[00:05:06] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:06] Build completed unsuccessfully in 0:00:31
[00:05:06] make: *** [prepare] Error 1
[00:05:07] Command failed. Attempt 2/5:
[00:05:08]     Updating crates.io index
[00:05:08]     Updating crates.io index
[00:05:08] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:08] Build completed unsuccessfully in 0:00:00
[00:05:08] make: *** [prepare] Error 1
[00:05:10] Command failed. Attempt 3/5:
[00:05:10]     Updating crates.io index
[00:05:10]     Updating crates.io index
[00:05:11] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:11] Build completed unsuccessfully in 0:00:00
[00:05:11] make: *** [prepare] Error 1
[00:05:14] Command failed. Attempt 4/5:
[00:05:14]     Updating crates.io index
[00:05:14]     Updating crates.io index
[00:05:15] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:15] Build completed unsuccessfully in 0:00:00
[00:05:15] make: *** [prepare] Error 1
[00:05:19] Command failed. Attempt 5/5:
[00:05:19]     Updating crates.io index
[00:05:19]     Updating crates.io index
[00:05:20] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:20] Build completed unsuccessfully in 0:00:00
[00:05:20] make: *** [prepare] Error 1
[00:05:20] The command has failed after 5 attempts.
travis_time:end:110c9f2c:start=1555083772692554127,finish=1555084093644517622,duration=320951963495
