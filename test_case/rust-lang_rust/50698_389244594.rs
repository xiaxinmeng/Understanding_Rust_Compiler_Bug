plain
####################################                                      50.8%
#####################################################                     73.6%
######################################################################## 100.0%
[00:01:14] extracting /checkout/obj/build/cache/2018-04-24/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:14] error: failed to parse lock file at: /checkout/src/Cargo.lock
[00:01:14] To learn more, run the command again with --verbose.
[00:01:14] To learn more, run the command again with --verbose.
[00:01:14] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:14] Build completed unsuccessfully in 0:00:19
[00:01:14] make: *** [prepare] Error 1
[00:01:14] Makefile:81: recipe for target 'prepare' failed
[00:01:15] Command failed. Attempt 2/5:
[00:01:15] error: failed to parse lock file at: /checkout/src/Cargo.lock
[00:01:15] To learn more, run the command again with --verbose.
[00:01:15] To learn more, run the command again with --verbose.
[00:01:15] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:15] Build completed unsuccessfully in 0:00:00
[00:01:15] Makefile:81: recipe for target 'prepare' failed
[00:01:15] make: *** [prepare] Error 1
[00:01:17] Command failed. Attempt 3/5:
[00:01:18] error: failed to parse lock file at: /checkout/src/Cargo.lock
[00:01:18] To learn more, run the command again with --verbose.
[00:01:18] To learn more, run the command again with --verbose.
[00:01:18] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:18] Build completed unsuccessfully in 0:00:00
[00:01:18] Makefile:81: recipe for target 'prepare' failed
[00:01:18] make: *** [prepare] Error 1
[00:01:21] Command failed. Attempt 4/5:
[00:01:21] error: failed to parse lock file at: /checkout/src/Cargo.lock
[00:01:21] To learn more, run the command again with --verbose.
[00:01:21] To learn more, run the command again with --verbose.
[00:01:21] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:21] Build completed unsuccessfully in 0:00:00
[00:01:21] make: *** [prepare] Error 1
[00:01:21] Makefile:81: recipe for target 'prepare' failed
[00:01:25] Command failed. Attempt 5/5:
[00:01:25] error: failed to parse lock file at: /checkout/src/Cargo.lock
[00:01:25] To learn more, run the command again with --verbose.
[00:01:25] To learn more, run the command again with --verbose.
[00:01:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:25] Build completed unsuccessfully in 0:00:00
[00:01:25] make: *** [prepare] Error 1
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:25] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:091f21c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
