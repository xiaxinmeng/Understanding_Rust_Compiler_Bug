plain
[00:05:28]    Compiling env_logger v0.5.8
[00:05:28]    Compiling parking_lot v0.5.4
[00:05:30]    Compiling flate2 v1.0.1
[00:05:30]    Compiling rls-data v0.15.0
[00:05:33] error[E0425]: cannot find value `post_order_rank` in this scope
[00:05:33]   --> librustc_data_structures/control_flow_graph/dominators/mod.rs:52:47
[00:05:33]    |
[00:05:33] 52 |                     new_idom = intersect_opt(&post_order_rank,
[00:05:33] 
[00:05:33] 
elease/deps/libserialize-c04ded78717d5d67.rlib --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-711907abcc246a2c.rlib --extern parking_lot_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot_core-8a3776c3686d27cd.rlib --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-69dd9e9383ee2661.rlib` (exit code: 101)
[00:05:35] error: build failed
[00:05:35] error: build failed
[00:05:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:35] expected success, got: exit code: 101
[00:05:35] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:05:35] travis_fold:end:stage0-rustc

[00:05:35] travis_time:end:stage0-rustc:start=1524634535630757454,finish=1524634571322470078,duration=35691712624


[00:05:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:35] Build completed unsuccessfully in 0:00:48
[00:05:35] Makefile:28: recipe for target 'all' failed
[00:05:35] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03a0edf5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
