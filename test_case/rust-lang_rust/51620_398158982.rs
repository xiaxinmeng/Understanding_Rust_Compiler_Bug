plain
[00:01:46]    Compiling serde_derive_internals v0.23.1
[00:01:50]    Compiling serde_derive v1.0.40
[00:01:57]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:02:33]     Finished dev [unoptimized] target(s) in 69.11 secs
[00:02:33] thread 'main' panicked at 'bootstrapping from a dev compiler in a stable release, but should only be bootstrapping from a released compiler!', bootstrap/sanity.rs:261:13
[00:02:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:02:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:02:33] make: *** [prepare] Error 1
[00:02:33] make: *** [prepare] Error 1
[00:02:33] Makefile:81: recipe for target 'prepare' failed
[00:02:33]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:02:33]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:02:33] thread 'main' panicked at 'bootstrapping from a dev compiler in a stable release, but should only be bootstrapping from a released compiler!', bootstrap/sanity.rs:261:13
[00:02:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:02:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:02:33] Build completed unsuccessfully in 0:00:00
[00:02:33] Makefile:81: recipe for target 'prepare' failed
[00:02:33] make: *** [prepare] Error 1
[00:02:34]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:02:34]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:02:34] thread 'main' panicked at 'bootstrapping from a dev compiler in a stable release, but should only be bootstrapping from a released compiler!', bootstrap/sanity.rs:261:13
[00:02:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:02:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:02:34] Build completed unsuccessfully in 0:00:00
[00:02:34] Makefile:81: recipe for target 'prepare' failed
[00:02:34] make: *** [prepare] Error 1
[00:02:34]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:02:34]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:02:34] thread 'main' panicked at 'bootstrapping from a dev compiler in a stable release, but should only be bootstrapping from a released compiler!', bootstrap/sanity.rs:261:13
[00:02:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:02:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:02:34] Build completed unsuccessfully in 0:00:00
[00:02:34] Makefile:81: recipe for target 'prepare' failed
[00:02:34] make: *** [prepare] Error 1
[00:02:35]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:02:35]     Finished dev [unoptimized] target(s) in 0.0 secs
[00:02:35] thread 'main' panicked at 'bootstrapping from a dev compiler in a stable release, but should only be bootstrapping from a released compiler!', bootstrap/sanity.rs:261:13
[00:02:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:02:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:02:35] Build completed unsuccessfully in 0:00:00
[00:02:35] Makefile:81: recipe for target 'prepare' failed
[00:02:35] make: *** [prepare] Error 1
[00:02:35] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:10e1540a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
