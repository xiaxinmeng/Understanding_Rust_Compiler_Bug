plain
Resolving deltas: 100% (615842/615842), completed with 4901 local objects.
---
[00:00:42] configure: rust.quiet-tests     := True
---
[00:31:59] error[E0423]: expected function, found struct `ty::Binder`
[00:31:59]    --> librustdoc/clean/auto_trait.rs:289:26
[00:31:59]     |
[00:31:59] 289 |         let trait_pred = ty::Binder(trait_ref);
[00:31:59]     |                          ^^^^^^^^^^ did you mean `ty::Binder { /* fields */ }`?
[00:31:59]
[00:31:59] error[E0423]: expected function, found struct `ty::Binder`
[00:31:59]    --> librustdoc/clean/auto_trait.rs:625:30
[00:31:59]     |
[00:31:59] 625 |         predicates.push_back(ty::Binder(ty::TraitPredicate {
[00:31:59]     |                              ^^^^^^^^^^ did you mean `ty::Binder { /* fields */ }`?
[00:31:59]
[00:31:59] error[E0532]: expected tuple struct/variant, found struct `ty::Binder`
[00:31:59]     --> librustdoc/clean/mod.rs:2841:25
[00:31:59]      |
[00:31:59] 2841 |                     for ty::Binder(ref pb) in obj.projection_bounds() {
[00:31:59]      |                         ^^^^^^^^^^ did you mean `ty::Binder { /* fields */ }`?
[00:31:59]
[00:32:01] error[E0616]: field `0` of struct `rustc::ty::Binder` is private
[00:32:01]    --> librustdoc/clean/mod.rs:123:9
[00:32:01]     |
[00:32:01] 123 |         self.0.clean(cx)
[00:32:01]     |         ^^^^^^
[00:32:01]
[00:32:04] error[E0616]: field `0` of struct `rustc::ty::Binder` is private
[00:32:04]     --> librustdoc/clean/mod.rs:2849:42
[00:32:04]      |
[00:32:04] 2849 |                         false, bindings, principal.0.substs);
[00:32:04]      |                                          ^^^^^^^^^^^
[00:32:04]
[00:32:05] error[E0616]: field `0` of struct `rustc::ty::Binder` is private
[00:32:05]    --> librustdoc/clean/simplify.rs:157:16
[00:32:05]     |
[00:32:05] 157 |             if pred.0.trait_ref.self_ty().is_self() {
[00:32:05]     |                ^^^^^^
[00:32:05]
[00:32:05] thread 'main' panicked at 'no index for a field', libcore/option.rs:914:5
---
[00:32:06] Makefile:28: recipe for target 'all' failed
[00:32:06] make: *** [all] Error 1
travis_time:end:Tue, 17 Apr 2018 04:18:12 GMT
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
