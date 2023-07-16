plain
[00:37:45]    Compiling aho-corasick v0.6.4
[00:37:53]    Compiling tempdir v0.3.7
[00:38:29]    Compiling minifier v0.0.11
[00:38:32]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:38:38] error[E0599]: no method named `clean` found for type `(&[rustc::hir::Ty], A)` in the current scope
[00:38:38]     --> librustdoc/clean/mod.rs:2169:50
[00:38:38]      |
[00:38:38] 2169 |             inputs: (&self.0.inputs[..], self.1).clean(cx),
[00:38:38]      |
[00:38:38]      = help: items from traits can only be used if the trait is implemented and in scope
[00:38:38]      = help: items from traits can only be used if the trait is implemented and in scope
[00:38:38]      = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:38:38]              candidate #1: `clean::Clean`
[00:38:38] 
[00:38:39] error[E0599]: no method named `into_inner` found for type `rustc::hir::Ty` in the current scope
[00:38:39]     --> librustdoc/clean/mod.rs:2877:75
[00:38:39]      |
[00:38:39] 2877 |                                         ty_substs.insert(ty_param_def, ty.into_inner().clean(cx));
[00:38:39] 
[00:38:42] error[E0308]: mismatched types
[00:38:42]    --> librustdoc/clean/auto_trait.rs:274:37
[00:38:42]     |
[00:38:42]     |
[00:38:42] 274 |             types: HirVec::from_vec(types),
[00:38:42]     |                                     ^^^^^ expected struct `rustc::hir::Ty`, found struct `syntax::ptr::P`
[00:38:42]     |
[00:38:42]     = note: expected type `std::vec::Vec<rustc::hir::Ty>`
[00:38:42]                found type `std::vec::Vec<syntax::ptr::P<rustc::hir::Ty>>`
[00:38:48] error: aborting due to 3 previous errors
[00:38:48] 
[00:38:48] Some errors occurred: E0308, E0599.
[00:38:48] For more information about an error, try `rustc --explain E0308`.
---
[00:38:48] 
[00:38:48] 
[00:38:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:38:48] Build completed unsuccessfully in 0:33:38
[00:38:48] Makefile:28: recipe for target 'all' failed
[00:38:48] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a675760
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
