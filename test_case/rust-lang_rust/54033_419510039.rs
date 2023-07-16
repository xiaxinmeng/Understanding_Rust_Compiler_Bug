plain
[00:22:18]    Compiling rustc_cratesio_shim v0.0.0 (file:///checkout/src/librustc_cratesio_shim)
[00:22:19]    Compiling ena v0.9.3
[00:22:19]    Compiling jobserver v0.1.11
[00:22:19]    Compiling smallvec v0.6.5
[00:22:19] error[E0277]: the trait bound `S: unify::backing_vec::UnificationStore` is not satisfied
[00:22:19]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.9.3/src/unify/backing_vec.rs:10:1
[00:22:19]    |
[00:22:19] 10 | type Key<S> = <S as UnificationStore>::Key;
[00:22:19]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `unify::backing_vec::UnificationStore` is not implemented for `S`
[00:22:19]    |
[00:22:19]    = help: consider adding a `where S: unify::backing_vec::UnificationStore` bound
[00:22:19] 
[00:22:19] error[E0277]: the trait bound `K: unify::UnifyKey` is not satisfied
[00:22:19]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.9.3/src/unify/mod.rs:184:1
[00:22:19]     |
[00:22:19] 184 | pub type InPlaceUnificationTable<K> = UnificationTable<InPlace<K>>;
[00:22:19]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `unify::UnifyKey` is not implemented for `K`
[00:22:19]     |
[00:22:19]     = help: consider adding a `where K: unify::UnifyKey` bound
[00:22:19] note: required by `unify::backing_vec::InPlace`
[00:22:19]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.9.3/src/unify/backing_vec.rs:50:1
[00:22:19]     |
[00:22:19] 50  | pub struct InPlace<K: UnifyKey> {
[00:22:19] 
[00:22:19] error: aborting due to 2 previous errors
[00:22:19] 
[00:22:19] For more information about this error, try `rustc --explain E0277`.
[00:22:19] For more information about this error, try `rustc --explain E0277`.
[00:22:19] error: Could not compile `ena`.
[00:22:19] 
[00:22:19] Caused by:
[00:22:19]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name ena /cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.9.3/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=8f2716632ba73697 -C extra-filename=-8f2716632ba73697 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-dea5dd9d4edbf91b.rlib --cap-lints allow` (exit code: 1)
[00:22:21] error: build failed
[00:22:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:22:21] expected success, got: exit code: 101
[00:22:21] expected success, got: exit code: 101
[00:22:21] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:22:21] travis_fold:end:stage1-rustc

[00:22:21] travis_time:end:stage1-rustc:start=1536341178861776308,finish=1536341202775854041,duration=23914077733


[00:22:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:21] Build completed unsuccessfully in 0:17:21
[00:22:21] Makefile:28: recipe for target 'all' failed
[00:22:21] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:001d2096
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:111c6536:start=1536341203544105768,finish=1536341203553045232,duration=8939464
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00601986
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:st
