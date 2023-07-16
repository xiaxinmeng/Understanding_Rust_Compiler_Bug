plain
[00:33:07]    Compiling aho-corasick v0.6.4
[00:33:13]    Compiling tempdir v0.3.7
[00:33:45]    Compiling minifier v0.0.11
[00:33:48]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:33:54] error[E0609]: no field `bounds` on type `&rustc::hir::ItemId`
[00:33:54]     --> librustdoc/clean/mod.rs:2900:78
[00:33:54]      |
[00:33:54] 2900 |             TyImplTraitExistential(ref exist_ty, _, _) => ImplTrait(exist_ty.bounds.clean(cx)),
[00:33:54] 
[00:34:02] error: aborting due to previous error
[00:34:02] 
[00:34:02] For more information about this error, try `rustc --explain E0609`.
---
[00:34:02] 
[00:34:02] 
[00:34:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:34:02] Build completed unsuccessfully in 0:29:22
[00:34:02] make: *** [all] Error 1
[00:34:02] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05fd7208
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
