plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:2c771ba4
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
###                                                                        5.3%
######################################################################## 100.0%
[00:04:16] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:04:16]     Updating crates.io index
[00:04:22] error: the lock file /checkout/src/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:22] Build completed unsuccessfully in 0:00:23
[00:04:22] make: *** [prepare] Error 1
[00:04:23] Command failed. Attempt 2/5:
[00:04:23]     Updating crates.io index
[00:04:23]     Updating crates.io index
[00:04:24] error: the lock file /checkout/src/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:24] Build completed unsuccessfully in 0:00:00
[00:04:24] make: *** [prepare] Error 1
[00:04:26] Command failed. Attempt 3/5:
[00:04:26]     Updating crates.io index
[00:04:26]     Updating crates.io index
[00:04:26] error: the lock file /checkout/src/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:26] Build completed unsuccessfully in 0:00:00
[00:04:26] make: *** [prepare] Error 1
[00:04:29] Command failed. Attempt 4/5:
[00:04:30]     Updating crates.io index
[00:04:30]     Updating crates.io index
[00:04:30] error: the lock file /checkout/src/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:30] Build completed unsuccessfully in 0:00:00
[00:04:30] make: *** [prepare] Error 1
          768M     0  768M   0% /var/ramfs
2616972 .
