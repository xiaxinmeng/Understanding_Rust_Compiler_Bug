plain
[00:01:44] travis_time:end:024c2352:start=1525813338496524957,finish=1525813338531377276,duration=34852319
travis_fold:start:make-prepare
travis_time:start:00073ee0
Attempting with retry: make prepare
[00:01:45] downloading https://dev-static.rust-lang.org/dist/2018-05-07/rust-std-1.26.0-x86_64-unknown-linux-gnu.tar.gz
                                                                           0.4%
                                                                           1.2%
#                                                                          2.5%
###                                                                        4.6%
---
###############################################################           88.0%
####################################################################      94.5%
######################################################################## 100.0%
[00:01:51] extracting /checkout/obj/build/cache/2018-05-07/rust-std-1.26.0-x86_64-unknown-linux-gnu.tar.gz
[00:01:51] downloading https://dev-static.rust-lang.org/dist/2018-05-07/rustc-1.26.0-x86_64-unknown-linux-gnu.tar.gz
                                                                           0.0%
                                                                           0.2%
                                                                           0.5%
                                                                           0.8%
---
###########################################################               82.8%
##################################################################        93.0%
######################################################################## 100.0%
[00:01:59] extracting /checkout/obj/build/cache/2018-05-07/rustc-1.26.0-x86_64-unknown-linux-gnu.tar.gz
[00:01:59] downloading https://dev-static.rust-lang.org/dist/2018-05-07/cargo-0.27.0-x86_64-unknown-linux-gnu.tar.gz
                                                                           0.3%
##############################################################            86.6%
######################################################################## 100.0%
[00:02:03] extracting /checkout/obj/build/cache/2018-05-07/cargo-0.27.0-x86_64-unknown-linux-gnu.tar.gz
[00:02:03] extracting /checkout/obj/build/cache/2018-05-07/cargo-0.27.0-x86_64-unknown-linux-gnu.tar.gz
[00:02:06]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:24] Build completed unsuccessfully in 0:00:39
[00:02:24] make: *** [prepare] Error 1
[00:02:24] Makefile:81: recipe for target 'prepare' failed
[00:02:24]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:24]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:24] Build completed unsuccessfully in 0:00:00
[00:02:24] Makefile:81: recipe for target 'prepare' failed
[00:02:24] make: *** [prepare] Error 1
[00:02:25]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:25]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:25] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:25] Build completed unsuccessfully in 0:00:01
[00:02:25] make: *** [prepare] Error 1
[00:02:25] Makefile:81: recipe for target 'prepare' failed
[00:02:26]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:26]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:27] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:27] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:27] Build completed unsuccessfully in 0:00:01
[00:02:27] make: *** [prepare] Error 1
[00:02:27] Makefile:81: recipe for target 'prepare' failed
[00:02:27]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:27]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:28] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:28] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:28] Build completed unsuccessfully in 0:00:01
[00:02:28] Makefile:81: recipe for target 'prepare' failed
[00:02:28] make: *** [prepare] Error 1
[00:02:28] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:142a9b5c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
