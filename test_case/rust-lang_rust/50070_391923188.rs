plain
[00:07:25]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:44]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:13:35]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:35]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:36] error[E0532]: expected tuple struct/variant, found struct `ty::Binder`
[00:13:36]   --> librustc_typeck/outlives/explicit.rs:53:45
[00:13:36]    |
[00:13:36] 53 |                 ty::Predicate::TypeOutlives(ty::Binder(ty::OutlivesPredicate(ty1, region2))) => {
[00:13:36]    |                                             ^^^^^^^^^^ did you mean `ty::Binder { /* fields */ }`?
[00:13:36] 
[00:13:36] error[E0532]: expected tuple struct/variant, found struct `ty::Binder`
[00:13:36]   --> librustc_typeck/outlives/explicit.rs:62:47
[00:13:36]    |
[00:13:36] 62 |                 ty::Predicate::RegionOutlives(ty::Binder(ty::OutlivesPredicate(
[00:13:36]    |                                               ^^^^^^^^^^ did you mean `ty::Binder { /* fields */ }`?
[00:13:36] 
[00:13:36] error[E0532]: expected tuple struct/variant, found struct `ty::Binder`
[00:13:36]   --> librustc_typeck/outlives/mod.rs:57:63
[00:13:36]    |
[00:13:36] 57 |                                 ty::Predicate::RegionOutlives(ty::Binder(p)) => {
[00:13:36]    |                                                               ^^^^^^^^^^ did you mean `ty::Binder { /* fields */ }`?
[00:13:36] 
[00:13:36] error[E0532]: expected tuple struct/variant, found struct `ty::Binder`
[00:13:36]   --> librustc_typeck/outlives/mod.rs:62:61
[00:13:36]    |
[00:13:36] 62 |                                 ty::Predicate::TypeOutlives(ty::Binder(p)) => {
[00:13:36]    |                                                             ^^^^^^^^^^ did you mean `ty::Binder { /* fields */ }`?
[00:13:36] 
[00:13:43] error[E0599]: no method named `substs` found for type `rustc::ty::Binder<rustc::ty::TraitRef<'_>>` in the current scope
[00:13:43]    --> librustc_typeck/outlives/implicit_infer.rs:211:60
[00:13:43]     |
[00:13:43] 211 |                         ex_trait_ref.with_self_ty(tcx, ty).substs(),
[00:13:43] 
[00:13:43] 
[00:13:43] error[E0599]: no method named `mk_param` found for type `rustc::ty::TyCtxt<'_, 'tcx, 'tcx>` in the current scope
[00:13:43]   --> librustc_typeck/outlives/utils.rs:80:48
[00:13:43]    |
[00:13:43] 80 |                         let ty: Ty<'tcx> = tcx.mk_param(param_ty.idx, param_ty.name);
[00:13:43] 
[00:13:43] error: aborting due to 6 previous errors
[00:13:43] 
[00:13:43] Some errors occurred: E0532, E0599.
[00:13:43] Some errors occurred: E0532, E0599.
[00:13:43] For more information about an error, try `rustc --explain E0532`.
[00:13:43] error: Could not compile `rustc_typeck`.
[00:13:43] 
[00:13:43] Caused by:
[00:13:43]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=519dd0906e1389d3 -C extra-filename=-519dd0906e1389d3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-6f643d03661f77af.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5c21a695e1301573.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-c7310be923214c6b.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-2bee8f0c1e719b79.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a7e6c70c1d8dd47c.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-c2f2c8c35b176352.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-a11aa962d27a56e4.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a8f5cae0510be7d3.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-cfccf57e1967c508.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-bde077c9625bcd15.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-f085762345e9053e/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c0082fee642cc0bf/out` (exit code: 101)
[00:16:25] error: build failed
[00:16:25] error: build failed
[00:16:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:25] expected success, got: exit code: 101
[00:16:25] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:16:25] travis_fold:end:stage0-rustc

[00:16:25] travis_time:end:stage0-rustc:start=1527214856072572671,finish=1527215539351725181,duration=683279152510


[00:16:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:25] Build completed unsuccessfully in 0:11:35
[00:16:25] Makefile:28: recipe for target 'all' failed
[00:16:25] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02c27f25
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
