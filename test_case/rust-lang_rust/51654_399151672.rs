plain
[00:39:07]    Compiling aho-corasick v0.6.4
[00:39:15]    Compiling tempfile v3.0.2
[00:39:51]    Compiling minifier v0.0.11
[00:39:53]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:40:00] error[E0599]: no method named `clean` found for type `(&[rustc::hir::Ty], A)` in the current scope
[00:40:00]     --> librustdoc/clean/mod.rs:2169:50
[00:40:00]      |
[00:40:00] 2169 |             inputs: (&self.0.inputs[..], self.1).clean(cx),
[00:40:00]      |
[00:40:00]      = help: items from traits can only be used if the trait is implemented and in scope
[00:40:00]      = help: items from traits can only be used if the trait is implemented and in scope
[00:40:00]      = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:40:00]              candidate #1: `clean::Clean`
[00:40:00] 
[00:40:01] error[E0599]: no method named `into_inner` found for type `rustc::hir::Ty` in the current scope
[00:40:01]     --> librustdoc/clean/mod.rs:2877:75
[00:40:01]      |
[00:40:01] 2877 |                                         ty_substs.insert(ty_param_def, ty.into_inner().clean(cx));
[00:40:01] 
[00:40:04] error[E0308]: mismatched types
[00:40:04]    --> librustdoc/clean/auto_trait.rs:274:37
[00:40:04]     |
[00:40:04]     |
[00:40:04] 274 |             types: HirVec::from_vec(types),
[00:40:04]     |                                     ^^^^^ expected struct `rustc::hir::Ty`, found struct `syntax::ptr::P`
[00:40:04]     |
[00:40:04]     = note: expected type `std::vec::Vec<rustc::hir::Ty>`
[00:40:04]                found type `std::vec::Vec<syntax::ptr::P<rustc::hir::Ty>>`
[00:40:10] error: aborting due to 3 previous errors
[00:40:10] 
[00:40:10] Some errors occurred: E0308, E0599.
[00:40:10] For more information about an error, try `rustc --explain E0308`.
---
[00:40:10] 
[00:40:10] 
[00:40:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:40:10] Build completed unsuccessfully in 0:33:38
[00:40:10] make: *** [all] Error 1
[00:40:10] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06c48b49
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
