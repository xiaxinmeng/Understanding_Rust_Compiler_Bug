plain
[00:06:20]    Compiling flate2 v1.0.1
[00:06:20]    Compiling rustc-rayon v0.1.0
[00:06:23]    Compiling backtrace v0.3.6
[00:06:26]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:26] error[E0425]: cannot find function `f` in this scope
[00:06:26]    --> librustc_data_structures/sync.rs:110:44
[00:06:26]     |
[00:06:26] 110 |                 WorkerLocal(OneThread::new(f(0)))
[00:06:26]     |                                            ^ not found in this scope
[00:06:26] 
[00:06:28] error[E0599]: no method named `into_inner` found for type `sync::OneThread<T>` in the current scope
[00:06:28]    --> librustc_data_structures/sync.rs:116:29
[00:06:28]     |
[00:06:28] 116 |                 vec![self.0.into_inner()]
[00:06:28] ...
[00:06:28] ...
[00:06:28] 664 | pub struct OneThread<T> {
[00:06:28]     | ----------------------- method `into_inner` not found for this
[00:06:28]     |
[00:06:28]     = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
[00:06:28]     = help: try with `sync::OneThread<T>::into_inner`
[00:06:28] note: candidate #1 is defined in an impl for the type `sync::OneThread<_>`
[00:06:28]    --> librustc_data_structures/sync.rs:692:5
[00:06:28]     |
[00:06:28] 692 |     pub fn into_inner(value: Self) -> T {
[00:06:28]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:28]     = help: items from traits can only be used if the trait is implemented and in scope
[00:06:28]     = note: the following trait defines an item `into_inner`, perhaps you need to implement it:
[00:06:28]             candidate #1: `std::sys_common::IntoInner`
[00:06:28] error: aborting due to 2 previous errors
[00:06:28] 
[00:06:28] Some errors occurred: E0425, E0599.
[00:06:28] For more information about an error, try `rustc --explain E0425`.
[00:06:28] For more information about an error, try `rustc --explain E0425`.
[00:06:28] error: Could not compile `rustc_data_structures`.
[00:06:28] 
[00:06:28] Caused by:
[00:06:28]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_data_structures librustc_data_structures/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=11aee7c39eccbe1e -C extra-filename=-11aee7c39eccbe1e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-a7895f6d3b02c33b.so --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-d0e6b5493278915a.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-8b6f4d00865d65fb.rlib --extern parking_lot_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot_core-b05cf9359a374184.rlib --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-1a278e44f37cc04f.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.rlib --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-92e4ca335c6450c6.rlib --extern ena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libena-ca956a2661daa1d2.rlib --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-389a492e2e51e633.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4f0866e958f59455.rlib` (exit code: 101)
[00:06:32] error: build failed
[00:06:32] error: build failed
[00:06:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:32] expected success, got: exit code: 101
[00:06:32] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:06:32] travis_fold:end:stage0-rustc

[00:06:32] travis_time:end:stage0-rustc:start=1526219582454547459,finish=1526219639361207359,duration=56906659900


[00:06:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:32] Build completed unsuccessfully in 0:01:12
[00:06:32] Makefile:28: recipe for target 'all' failed
[00:06:32] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2137fabe
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
