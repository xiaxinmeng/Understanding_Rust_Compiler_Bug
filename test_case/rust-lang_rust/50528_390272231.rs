plain
[00:18:23]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:18:23] error: unused import: `attr`
[00:18:23]   --> librustc_metadata/cstore.rs:23:19
[00:18:23]    |
[00:18:23] 23 | use syntax::{ast, attr};
[00:18:23]    |
[00:18:23]    = note: `-D unused-imports` implied by `-D warnings`
[00:18:23] 
[00:18:23] 
[00:18:23] error: unused import: `syntax::edition::Edition`
[00:18:23]   --> librustc_metadata/cstore.rs:24:5
[00:18:23]    |
[00:18:23] 24 | use syntax::edition::Edition;
[00:18:23] 
[00:18:23] 
[00:18:24] error[E0599]: no method named `edition` found for type `std::rc::Rc<cstore::CrateMetadata>` in the current scope
[00:18:24]    --> librustc_metadata/cstore_impl.rs:522:55
[00:18:24]     |
[00:18:24] 522 |                                                  data.edition());
[00:18:24] 
[00:18:25] error: aborting due to 3 previous errors
[00:18:25] 
[00:18:25] For more information about this error, try `rustc --explain E0599`.
[00:18:25] For more information about this error, try `rustc --explain E0599`.
[00:18:25] error: Could not compile `rustc_metadata`.
[00:18:25] 
[00:18:25] Caused by:
[00:18:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_metadata librustc_metadata/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=d6882130f6bbfec0 -C extra-filename=-d6882130f6bbfec0 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-949c6ac74d89c4f0.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a7e6c70c1d8dd47c.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-7a064031021abb10.rlib --extern syntax_ext=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-6f46c57304c968a4.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-b6248f90fefe05c1.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-bd8d9cf8bca9bb0a.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a476bbed33563995.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a476bbed33563995.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-ef6ca4ebda266b83.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-01d5fdf06e96971b.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-bde077c9625bcd15.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-029c79b35b3d2152.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c0082fee642cc0bf/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-f085762345e9053e/out/.libs` (exit code: 101)
[00:22:48] error: build failed
[00:22:48] error: build failed
[00:22:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:22:48] expected success, got: exit code: 101
[00:22:48] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:22:48] travis_fold:end:stage0-rustc

[00:22:48] travis_time:end:stage0-rustc:start=1526662390524332622,finish=1526663386395575680,duration=995871243058


[00:22:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:48] Build completed unsuccessfully in 0:16:53
[00:22:48] Makefile:28: recipe for target 'all' failed
[00:22:48] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d7a5b2e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
