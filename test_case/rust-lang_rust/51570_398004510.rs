plain
[00:22:47]    Compiling chalk-engine v0.6.0
[00:22:49]    Compiling parking_lot_core v0.2.14
[00:22:49]    Compiling tempdir v0.3.7
[00:22:49]    Compiling env_logger v0.5.8
[00:22:49] error[E0597]: borrowed value does not live long enough
[00:22:49]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot_core-0.2.14/src/word_lock.rs:66:41
[00:22:49]    |
[00:22:49] 66 |         if let Some(tls) = try_get_tls(&THREAD_DATA) {
[00:22:49]    |                                         ^^^^^^^^^^^ temporary value does not live long enough
[00:22:49] 69 |     }
[00:22:49] 69 |     }
[00:22:49]    |     - temporary value only lives until here
[00:22:49]    |
[00:22:49]    = note: borrowed value must be valid for the static lifetime...
[00:22:49] 
[00:22:49] error[E0597]: borrowed value does not live long enough
[00:22:49]     |
[00:22:49]     |
[00:22:49] 178 |     if let Some(tls) = try_get_tls(&THREAD_DATA) {
[00:22:49]     |                                     ^^^^^^^^^^^ temporary value does not live long enough
[00:22:49] 179 |         return &*tls;
[00:22:49] 180 |     }
[00:22:49]     |     - temporary value only lives until here
[00:22:49]     |
[00:22:49]     = note: borrowed value must be valid for the static lifetime...
[00:22:49]     = note: consider using a `let` binding to increase its lifetime
[00:22:49] error: aborting due to 2 previous errors
[00:22:49] 
[00:22:49] For more information about this error, try `rustc --explain E0597`.
[00:22:49] error: Could not compile `parking_lot_core`.
[00:22:49] error: Could not compile `parking_lot_core`.
[00:22:49] 
[00:22:49] Caused by:
[00:22:49]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name parking_lot_core /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot_core-0.2.14/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 --cfg feature="nightly" -C metadata=688af4b61ce9ef3e -C extra-filename=-688af4b61ce9ef3e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librand-38bd483223ea337c.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-569efe1a6d869259.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-a3ea060267c173a1.rlib --cap-lints allow` (exit code: 101)
[00:22:52] error: build failed
[00:22:52] error: build failed
[00:22:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:22:52] expected success, got: exit code: 101
[00:22:52] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:22:52] travis_fold:end:stage1-rustc

[00:22:52] travis_time:end:stage1-rustc:start=1529316064385206424,finish=1529316089308340723,duration=24923134299


[00:22:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:52] Build completed unsuccessfully in 0:17:58
[00:22:52] make: *** [all] Error 1
[00:22:52] Makefile:28: recipe for target 'all' failed
