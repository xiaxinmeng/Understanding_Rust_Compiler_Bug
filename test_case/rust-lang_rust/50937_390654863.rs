plain
####                                                                       6.7%
##############################################################            87.3%
######################################################################## 100.0%
[00:01:16] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:16] error: failed to read `/chalk/chalk-engine/Cargo.toml`
[00:01:16] Caused by:
[00:01:16]   No such file or directory (os error 2)
[00:01:16]   No such file or directory (os error 2)
[00:01:16] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:16] Build completed unsuccessfully in 0:00:14
[00:01:16] make: *** [prepare] Error 1
[00:01:16] Makefile:81: recipe for target 'prepare' failed
[00:01:17] Command failed. Attempt 2/5:
[00:01:17] error: failed to read `/chalk/chalk-engine/Cargo.toml`
[00:01:17] Caused by:
[00:01:17]   No such file or directory (os error 2)
[00:01:17]   No such file or directory (os error 2)
[00:01:17] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:17] Build completed unsuccessfully in 0:00:00
[00:01:17] make: *** [prepare] Error 1
[00:01:17] Makefile:81: recipe for target 'prepare' failed
[00:01:19] Command failed. Attempt 3/5:
[00:01:19] error: failed to read `/chalk/chalk-engine/Cargo.toml`
[00:01:19] Caused by:
[00:01:19]   No such file or directory (os error 2)
[00:01:19]   No such file or directory (os error 2)
[00:01:19] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:19] Build completed unsuccessfully in 0:00:00
[00:01:19] Makefile:81: recipe for target 'prepare' failed
[00:01:19] make: *** [prepare] Error 1
[00:01:22] Command failed. Attempt 4/5:
[00:01:22] error: failed to read `/chalk/chalk-engine/Cargo.toml`
[00:01:22] Caused by:
[00:01:22]   No such file or directory (os error 2)
[00:01:22]   No such file or directory (os error 2)
[00:01:22] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:22] Build completed unsuccessfully in 0:00:00
[00:01:22] Makefile:81: recipe for target 'prepare' failed
[00:01:22] make: *** [prepare] Error 1
[00:01:26] Command failed. Attempt 5/5:
[00:01:26] error: failed to read `/chalk/chalk-engine/Cargo.toml`
[00:01:26] Caused by:
[00:01:26]   No such file or directory (os error 2)
[00:01:26]   No such file or directory (os error 2)
[00:01:26] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:26] Build completed unsuccessfully in 0:00:00
[00:01:26] make: *** [prepare] Error 1
[00:01:26] Makefile:81: recipe for target 'prepare' failed
[00:01:26] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0a91c3ee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
