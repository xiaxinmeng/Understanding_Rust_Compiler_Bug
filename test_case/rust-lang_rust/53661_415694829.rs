plain
[00:55:01] ....................................................................................................
[00:55:04] ....................................................................................................
[00:55:07] .........i..........................................................................................
[00:55:10] ....................................................................................................
[00:55:12] ..........................................................iiiiiiiii.................................
[00:55:18] ....................................................................................................
[00:55:22] ....................................................................................................
[00:55:25] .......................................i............................................................
[00:55:27] .........................................................................................i.i..ii....
---
[01:20:18]    Compiling rustc_driver v0.0.0 (file:///checkout/src/librustc_driver)
[01:20:19] error[E0425]: cannot find function `TyParam` in module `ty`
[01:20:19]    --> librustc_driver/test.rs:316:34
[01:20:19]     |
[01:20:19] 316 |         self.infcx.tcx.mk_ty(ty::TyParam(ty::GenericParam {
[01:20:19] help: possible candidate is found in another module, you can import it into scope
[01:20:19]     |
[01:20:19] 13  | use rustc::hir::def::Def::TyParam;
[01:20:19]     |
[01:20:19]     |
[01:20:19] 
[01:20:19] error[E0560]: struct `rustc::ty::GenericParam` has no field named `idx`
[01:20:19]    --> librustc_driver/test.rs:317:13
[01:20:19]     |
[01:20:19] 317 |             idx: index,
[01:20:19]     |             ^^^ `rustc::ty::GenericParam` does not have this field
[01:20:19]     |
[01:20:19]     = note: available fields are: `def_id`, `index`
[01:20:19] 
[01:20:19] error[E0560]: struct `rustc::ty::GenericParam` has no field named `name`
[01:20:19]    --> librustc_driver/test.rs:327:13
[01:20:19] 327 |             name,
[01:20:19] 327 |             name,
[01:20:19]     |             ^^^^ `rustc::ty::GenericParam` does not have this field
[01:20:19]     |
[01:20:19]     = note: available fields are: `def_id`, `index`
[01:20:20] error: aborting due to 3 previous errors
[01:20:20] 
[01:20:20] Some errors occurred: E0425, E0560.
[01:20:20] For more information about an error, try `rustc --explain E0425`.
[01:20:20] For more information about an error, try `rustc --explain E0425`.
[01:20:20] error: Could not compile `rustc_driver`.
[01:20:20] 
[01:20:20] To learn more, run the command again with --verbose.
[01:20:20] 
[01:20:20] 
[01:20:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:20:20] 
[01:20:20] 
[01:20:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:20] Build completed unsuccessfully in 0:29:27
[01:20:20] Build completed unsuccessfully in 0:29:27
[01:20:20] make: *** [check] Error 1
[01:20:20] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1db09b30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
