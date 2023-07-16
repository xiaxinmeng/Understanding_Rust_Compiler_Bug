plain
[00:06:58]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:13]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:12:39]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:12:39]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:12:40] error[E0252]: the name `ToRegionVid` is defined multiple times
[00:12:40]   --> librustc_mir/borrow_check/nll/constraint_generation.rs:16:5
[00:12:40]    |
[00:12:40] 13 | use borrow_check::nll::ToRegionVid;
[00:12:40]    |     ------------------------------ previous import of the trait `ToRegionVid` here
[00:12:40] ...
[00:12:40] 16 | use borrow_check::nll::ToRegionVid;
[00:12:40]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ToRegionVid` reimported here
[00:12:40]    |
[00:12:40]    = note: `ToRegionVid` must be defined only once in the type namespace of this module
[00:12:40] help: You can use `as` to change the binding name of the import
[00:12:40]    |
[00:12:40] 16 | use borrow_check::nll::ToRegionVid as OtherToRegionVid;
[00:12:40] 
[00:12:40] 
[00:12:41] error[E0603]: struct `OutlivesConstraint` is private
[00:12:41]   --> librustc_mir/borrow_check/nll/type_check/mod.rs:17:69
[00:12:41]    |
[00:12:41] 17 | use borrow_check::nll::region_infer::{ClosureRegionRequirementsExt, OutlivesConstraint, TypeTest};
[00:12:41] 
[00:12:41] 
[00:12:41] error: unused import: `borrow_check::nll::ToRegionVid`
[00:12:41]   --> librustc_mir/borrow_check/nll/constraint_generation.rs:16:5
[00:12:41]    |
[00:12:41] 16 | use borrow_check::nll::ToRegionVid;
[00:12:41]    |
[00:12:41]    = note: `-D unused-imports` implied by `-D warnings`
[00:12:41] 
[00:12:41] 
[00:12:41] error: unused import: `OutlivesConstraint`
[00:12:41]   --> librustc_mir/borrow_check/nll/type_check/mod.rs:17:69
[00:12:41]    |
[00:12:41] 17 | use borrow_check::nll::region_infer::{ClosureRegionRequirementsExt, OutlivesConstraint, TypeTest};
[00:12:41] 
[00:12:41] 
[00:12:43] error[E0599]: no method named `find_constraint` found for type `&std::rc::Rc<borrow_check::nll::region_infer::RegionInferenceContext<'_>>` in the current scope
[00:12:43]   --> librustc_mir/borrow_check/nll/explain_borrow/mod.rs:57:35
[00:12:43]    |
[00:12:43] 57 |         let region_sub = regioncx.find_constraint(borrow_region_vid, context.loc);
[00:12:43] 
[00:12:43] 
[00:12:43] error[E0599]: no method named `cause` found for type `borrow_check::nll::region_infer::values::RegionValues` in the current scope
[00:12:43]    --> librustc_mir/borrow_check/nll/region_infer/error_reporting.rs:270:35
[00:12:43]     |
[00:12:43] 270 |         self.liveness_constraints.cause(region_sub, elem)
[00:12:43]     | 
[00:12:43]     | 
[00:12:43]    ::: librustc_mir/borrow_check/nll/region_infer/values.rs:182:1
[00:12:43]     |
[00:12:43] 182 | pub(super) struct RegionValues {
[00:12:43]     | ------------------------------ method `cause` not found for this
[00:12:43]     = help: items from traits can only be used if the trait is implemented and in scope
[00:12:43]     = help: items from traits can only be used if the trait is implemented and in scope
[00:12:43]     = note: the following traits define an item `cause`, perhaps you need to implement one of them:
[00:12:43]             candidate #1: `std::error::Error`
[00:12:43]             candidate #2: `rustc::infer::lattice::LatticeDir`
[00:12:50] error: aborting due to 6 previous errors
[00:12:50] 
[00:12:50] Some errors occurred: E0252, E0599, E0603.
[00:12:50] For more information about an error, try `rustc --explain E0252`.
[00:12:50] For more information about an error, try `rustc --explain E0252`.
[00:12:50] error: Could not compile `rustc_mir`.
[00:12:50] 
[00:12:50] Caused by:
[00:12:50]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8fc603afb8ea9b13 -C extra-filename=-8fc603afb8ea9b13 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-d45628fe21047b42.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-d8b3f1986e621085.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-09e9dbd1ef48ffa5.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-b51deb005c05dd3b.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-6020508f01da724d.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-023d781fbd65d983.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-9c861d36e123bec8.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-2e546cbdf217aece.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-1dddb0fa9d8a512f.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-be9737b074a8dae0.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-25582ce13d3618ea.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e036f8f5b9204e52.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-066098b54e835a1f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1b02e0d020520ac.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f36f6cb494d373be.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-a59a4023b81a89b2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-ea5907e223517cf2/out` (exit code: 101)
[00:14:18] error: build failed
[00:14:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:18] expected success, got: exit code: 101
[00:14:18] expected success, got: exit code: 101
[00:14:18] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:14:18] travis_fold:end:stage0-rustc

[00:14:18] travis_time:end:stage0-rustc:start=1530485445002179018,finish=1530486008756174496,duration=563753995478


[00:14:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:18] Build completed unsuccessfully in 0:09:34
[00:14:18] make: *** [all] Error 1
[00:14:18] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1fb83dbd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0ca3a9ea:start=1530486009363045686,finish=1530486009370277946,duration=7232260
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00037948
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3a5bec98
$ dmesg | grep -i kill
