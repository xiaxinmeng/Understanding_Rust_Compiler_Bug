plain
[00:20:38]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:20:38]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:20:38]    Compiling cc v1.0.15
[00:20:39]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:20:48] error[E0433]: failed to resolve. Use of undeclared type or module `StableHasher`
[00:20:48]    --> librustc_codegen_llvm/debuginfo/metadata.rs:145:26
[00:20:48]     |
[00:20:48] 145 |         let mut hasher = StableHasher::<Fingerprint>::new();
[00:20:48]     |                          ^^^^^^^^^^^^ Use of undeclared type or module `StableHasher`
[00:20:48] 
[00:20:50] error[E0599]: no method named `hash_stable` found for type `&rustc::ty::TyS<'_>` in the current scope
[00:20:50]    --> librustc_codegen_llvm/debuginfo/metadata.rs:150:23
[00:20:50]     |
[00:20:50] 150 |                 type_.hash_stable(hcx, &mut hasher);
[00:20:50]     |
[00:20:50]     = help: items from traits can only be used if the trait is in scope
[00:20:50]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:20:50]             candidate #1: `use rustc_data_structures::stable_hasher::HashStable;`
[00:20:50]             candidate #1: `use rustc_data_structures::stable_hasher::HashStable;`
[00:20:50] 
[00:20:50] error[E0277]: the trait bound `str: std::marker::Sized` is not satisfied
[00:20:50]    --> librustc_codegen_llvm/debuginfo/metadata.rs:153:13
[00:20:50]     |
[00:20:50] 153 |         let unique_type_id = hasher.finish().to_hex();
[00:20:50]     |             ^^^^^^^^^^^^^^ `str` does not have a constant size known at compile-time
[00:20:50]     |
[00:20:50]     = help: the trait `std::marker::Sized` is not implemented for `str`
[00:20:50]     = note: all local variables must have a statically known size
[00:20:52] error: aborting due to 3 previous errors
[00:20:52] 
[00:20:52] Some errors occurred: E0277, E0433, E0599.
[00:20:52] For more information about an error, try `rustc --explain E0277`.
[00:20:52] For more information about an error, try `rustc --explain E0277`.
[00:20:52] error: Could not compile `rustc_codegen_llvm`.
[00:20:52] 
[00:20:52] Caused by:
[00:20:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="jemalloc" --cfg feature="rustc_target" -C metadata=b92357ade41ea302 -C extra-filename=-b92357ade41ea302 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-8c428dcf5a112e65.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-06f3d00d16d748ac.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-029c79b35b3d2152.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-f223cc35df6c29ec.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-63d08112eafc7ec5.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a7e6c70c1d8dd47c.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-bde077c9625bcd15.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-01d5fdf06e96971b.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-d18b6c0e8f47b341.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-ef6ca4ebda266b83.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-7a064031021abb10.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-5940c2a57172dcdb.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-bc18792c428d5db9.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-eeb182705d30c885.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-3024734921a8d3c2.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-b6248f90fefe05c1.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-b935674ebc6cf62f.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a476bbed33563995.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a476bbed33563995.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-188b86557d2f47a0.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-ae6ba972807bb99f.rlib --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-a11aa962d27a56e4.so --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-58e666292c0e8109.so --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-7598e70b4e56f241.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-949c6ac74d89c4f0.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c0082fee642cc0bf/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-f085762345e9053e/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-dcfd0659688e17a9/out -L native=/usr/lib/llvm-3.9/lib` (exit code: 101)
[00:20:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:20:52] expected success, got: exit code: 101
[00:20:52] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:20:52] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:20:52] travis_time:end:stage0-rustc_codegen_llvm:start=1526593229846976926,finish=1526593243921389416,duration=14074412490

[00:20:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:52] Build completed unsuccessfully in 0:15:47
[00:20:52] make: *** [all] Error 1
[00:20:52] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:097d1e82
2392656 .
---
150740 ./.git/modules/src
149124 ./src/llvm-emscripten/test
137068 ./obj/build/bootstrap/debug/incremental
122516 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
122512 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f14qqn02pe-122hgq-ki6mp0u53dvu
89804 ./src/llvm/test/CodeGen
76652 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
71856 ./.git/modules/src/tools
70944
