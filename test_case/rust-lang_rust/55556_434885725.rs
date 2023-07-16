plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0268fe51
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
######################################################################    98.5%
######################################################################## 100.0%
[00:05:57] extracting /checkout/obj/build/cache/2018-10-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:05:57]     Updating crates.io index
[00:06:02] error: the lock file needs to be updated but --locked was passed to prevent this
[00:06:02] Build completed unsuccessfully in 0:00:20
[00:06:02] make: *** [prepare] Error 1
[00:06:03] Command failed. Attempt 2/5:
[00:06:03] Command failed. Attempt 2/5:
[00:06:03] error: the lock file needs to be updated but --locked was passed to prevent this
[00:06:03] Build completed unsuccessfully in 0:00:00
[00:06:03] make: *** [prepare] Error 1
[00:06:05] Command failed. Attempt 3/5:
[00:06:05] Command failed. Attempt 3/5:
[00:06:06] error: the lock file needs to be updated but --locked was passed to prevent this
[00:06:06] Build completed unsuccessfully in 0:00:00
[00:06:06] make: *** [prepare] Error 1
[00:06:09] Command failed. Attempt 4/5:
[00:06:09] Command failed. Attempt 4/5:
[00:06:09] error: the lock file needs to be updated but --locked was passed to prevent this
[00:06:09] Build completed unsuccessfully in 0:00:00
[00:06:09] make: *** [prepare] Error 1
[00:06:13] Command failed. Attempt 5/5:
[00:06:13] Command failed. Attempt 5/5:
[00:06:13] error: the lock file needs to be updated but --locked was passed to prevent this
[00:06:13] Build completed unsuccessfully in 0:00:00
[00:06:13] make: *** [prepare] Error 1
[00:06:13] The command has failed after 5 attempts.
travis_time:end:016fc223:start=1541029393061469673,finish=1541029788608831637,duration=395547361964
