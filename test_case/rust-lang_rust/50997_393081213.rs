plain
[00:25:29]    Compiling rustc-rayon v0.1.0
[00:25:29]    Compiling backtrace v0.3.6
[00:25:36]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:25:39]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
s/libscoped_tls-947947ef42b5e09f.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f276f3b9f92eb810.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-7b5d7f90f8db7507.so` (exit code: 101)
[00:25:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:25:40] expected success, got: exit code: 101
[00:25:40] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:25:40] travis_fold:end:stage1-rustc

[00:25:40] travis_time:end:stage1-rustc:start=1527669932460685039,finish=1527669977721636543,duration=45260951504


[00:25:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:25:40] Build completed unsuccessfully in 0:20:32
[00:25:40] Makefile:28: recipe for target 'all' failed
[00:25:40] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0272fc34
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
