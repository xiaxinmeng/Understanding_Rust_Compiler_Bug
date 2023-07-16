plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0286e870
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:04:42] 
######################################################################## 100.0%
[00:04:42] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:04:42]     Updating crates.io index
[00:04:57]     Updating git repository `https://github.com/tkaitchuck/aHash.git`
[00:04:58] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:58] Build completed unsuccessfully in 0:00:32
[00:04:58] make: *** [prepare] Error 1
[00:04:59] Command failed. Attempt 2/5:
[00:04:59] Command failed. Attempt 2/5:
[00:04:59] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:59] Build completed unsuccessfully in 0:00:00
[00:04:59] make: *** [prepare] Error 1
[00:05:01] Command failed. Attempt 3/5:
[00:05:01] Command failed. Attempt 3/5:
[00:05:02] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:02] Build completed unsuccessfully in 0:00:00
[00:05:02] make: *** [prepare] Error 1
[00:05:05] Command failed. Attempt 4/5:
[00:05:05] Command failed. Attempt 4/5:
[00:05:05] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:05] Build completed unsuccessfully in 0:00:00
[00:05:05] make: *** [prepare] Error 1
[00:05:09] Command failed. Attempt 5/5:
[00:05:09] Command failed. Attempt 5/5:
[00:05:10] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:10] Build completed unsuccessfully in 0:00:00
[00:05:10] make: *** [prepare] Error 1
[00:05:10] The command has failed after 5 attempts.
travis_time:end:2e04681c:start=1554061599910155664,finish=1554061911979314457,duration=312069158793
