plain
#####################################                                     52.5%
###########################################################               83.2%
######################################################################## 100.0%
[00:01:20] extracting /checkout/obj/build/cache/2018-04-24/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:20] error: failed to parse lock file at: /checkout/src/Cargo.lock
[00:01:20] Caused by:
[00:01:20] Caused by:
[00:01:20]   could not parse input as TOML
[00:01:20] Caused by:
[00:01:20] Caused by:
[00:01:20]   expected an equals, found a comma at line 2223
[00:01:20] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:20] Build completed unsuccessfully in 0:00:17
[00:01:20] make: *** [prepare] Error 1
[00:01:20] Makefile:81: recipe for target 'prepare' failed
[00:01:21] Command failed. Attempt 2/5:
[00:01:21] error: failed to parse lock file at: /checkout/src/Cargo.lock
[00:01:21] Caused by:
[00:01:21] Caused by:
[00:01:21]   could not parse input as TOML
[00:01:21] Caused by:
[00:01:21] Caused by:
[00:01:21]   expected an equals, found a comma at line 2223
[00:01:21] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:21] Build completed unsuccessfully in 0:00:00
[00:01:21] Makefile:81: recipe for target 'prepare' failed
[00:01:21] make: *** [prepare] Error 1
[00:01:23] Command failed. Attempt 3/5:
[00:01:23] error: failed to parse lock file at: /checkout/src/Cargo.lock
[00:01:23] Caused by:
[00:01:23] Caused by:
[00:01:23]   could not parse input as TOML
[00:01:23] Caused by:
[00:01:23] Caused by:
[00:01:23]   expected an equals, found a comma at line 2223
[00:01:23] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:23] Build completed unsuccessfully in 0:00:00
[00:01:23] make: *** [prepare] Error 1
[00:01:23] Makefile:81: recipe for target 'prepare' failed
[00:01:26] Command failed. Attempt 4/5:
[00:01:26] error: failed to parse lock file at: /checkout/src/Cargo.lock
[00:01:26] Caused by:
[00:01:26] Caused by:
[00:01:26]   could not parse input as TOML
[00:01:26] Caused by:
[00:01:26] Caused by:
[00:01:26]   expected an equals, found a comma at line 2223
[00:01:26] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:26] Build completed unsuccessfully in 0:00:00
[00:01:26] Makefile:81: recipe for target 'prepare' failed
[00:01:26] make: *** [prepare] Error 1
[00:01:30] Command failed. Attempt 5/5:
[00:01:30] error: failed to parse lock file at: /checkout/src/Cargo.lock
[00:01:30] Caused by:
[00:01:30] Caused by:
[00:01:30]   could not parse input as TOML
[00:01:30] Caused by:
[00:01:30] Caused by:
[00:01:30]   expected an equals, found a comma at line 2223
[00:01:30] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:30] Build completed unsuccessfully in 0:00:00
[00:01:30] Makefile:81: recipe for target 'prepare' failed
[00:01:30] make: *** [prepare] Error 1
[00:01:30] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:05321106
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
