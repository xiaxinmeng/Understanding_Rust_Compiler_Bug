plain
[00:07:44]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:48] error[E0425]: cannot find value `r` in this scope
[00:07:48]    --> librustc/middle/resolve_lifetime.rs:421:14
[00:07:48]     |
[00:07:48] 421 |     Lrc::new(r)
[00:07:48]     |              ^ did you mean `P`?
[00:08:06] error[E0282]: type annotations needed
[00:08:06]     --> librustc/session/config.rs:2198:14
[00:08:06]      |
[00:08:06]      |
[00:08:06] 2180 |     let mut externs = BTreeMap::new();
[00:08:06]      |         ----------- consider giving `externs` a type
[00:08:06] ...
[00:08:06] 2198 |             .insert(location.to_string());
[00:08:06]      |              ^^^^^^ cannot infer type for `V`
[00:08:06]      = note: type must be known at this point
[00:08:06] 
[00:08:11] error[E0282]: type annotations needed
[00:08:11]     --> librustc/ty/context.rs:1139:30
[00:08:11]     --> librustc/ty/context.rs:1139:30
[00:08:11]      |
[00:08:11] 1134 |         let mut trait_map = FxHashMap();
[00:08:11]      |             ------------- consider giving `trait_map` a type
[00:08:11] ...
[00:08:11] 1139 |                             .insert(hir_id.local_id,
[00:08:11]      |                              ^^^^^^ cannot infer type for `T`
[00:08:11]      = note: type must be known at this point
[00:08:11] 
[00:08:12] error: aborting due to 3 previous errors
[00:08:12] 
[00:08:12] 
[00:08:12] Some errors occurred: E0282, E0425.
[00:08:12] For more information about an error, try `rustc --explain E0282`.
[00:08:12] error: Could not compile `rustc`.
[00:08:12] 
[00:08:12] Caused by:
[00:08:12]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=3969fd5fc27e256c -C extra-filename=-3969fd5fc27e256c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-5d5d2990686c15ff.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-38d27ec0a56fa3cb.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-693c666d349ae1b6.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-35cd99699a8927a8.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-6fb61f83cb288550.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-2b513184d9d612c2.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-67c8aaeb97e8d843.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-09a8465c475acbfe.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-d405c21c3677962b.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-a185b6f37f2c1490.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-96863e4e4a7d8742.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1436fcd917ac8cb6.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-7f18c5c76091a313.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-3856cda38a76ff6d.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-a4d022068ecc6660.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stag-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7205e6adb9d66e6b/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-27571ee36438df44/out` (exit code: 101)
[00:08:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:12] expected success, got: exit code: 101
[00:08:12] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:08:12] travis_fold:end:stage0-rustc

[00:08:12] travis_time:end:stage0-rustc:start=1532499131403659242,finish=1532499296985568400,duration=165581909158


[00:08:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:12] Build completed unsuccessfully in 0:03:44
[00:08:12] Makefile:28: recipe for target 'all' failed
[00:08:12] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21db2bad
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:071caed0:start=1532499297589684399,finish=1532499297598988550,duration=9304151
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2e4e53b2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03a163b0
travis_time:start:03a163b0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a9af09a
$ dmesg | grep -i kill
