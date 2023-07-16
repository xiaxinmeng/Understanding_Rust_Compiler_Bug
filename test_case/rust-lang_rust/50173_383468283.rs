plain
[00:06:52]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:08:51]    Compiling rustc_const_math v0.0.0 (file:///checkout/src/librustc_const_math)
[00:08:51]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:09:23]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:09:28] error[E0433]: failed to resolve. Use of undeclared type or module `std`
[00:09:28]    --> librustc/traits/object_safety.rs:323:24
[00:09:28]     |
[00:09:28] 323 |                 .chain(std::iter::once(&predicate))
[00:09:28]     |                        ^^^ Use of undeclared type or module `std`
[00:09:28] 
[00:09:29] error[E0412]: cannot find type `Predicate` in this scope
[00:09:29]    --> librustc/traits/object_safety.rs:322:36
[00:09:29]     |
[00:09:29] 322 |             let caller_bounds: Vec<Predicate<'tcx>> = param_env.caller_bounds.iter()
[00:09:29]     |                                    ^^^^^^^^^ not found in this scope
[00:09:29] help: possible candidate is found in another module, you can import it into scope
[00:09:29] 20  | use ty::Predicate;
[00:09:29]     |
[00:09:29] 
[00:10:03] error: unreachable statement
[00:10:03] error: unreachable statement
[00:10:03]    --> librustc/traits/object_safety.rs:314:9
[00:10:03]     |
[00:10:03] 314 | /         let param_env = {
[00:10:03] 315 | |             let mut param_env = self.param_env(method.def_id);
[00:10:03] 316 | |
[00:10:03] 317 | |             let predicate = ty::TraitRef {
[00:10:03] ...   |
[00:10:03] 328 | |             param_env
[00:10:03] 329 | |         };
[00:10:03]     |
[00:10:03]     |
[00:10:03]     = note: `-D unreachable-code` implied by `-D warnings`
[00:10:03] 
[00:10:03] error[E0599]: no method named `to_predicate` found for type `ty::sty::TraitRef<'_>` in the current scope
[00:10:03]    --> librustc/traits/object_safety.rs:320:15
[00:10:03]     |
[00:10:03] 320 |             }.to_predicate();
[00:10:03]     | 
[00:10:03]     | 
[00:10:03]    ::: librustc/ty/sty.rs:543:1
[00:10:03]     |
[00:10:03] 543 | pub struct TraitRef<'tcx> {
[00:10:03]     | ------------------------- method `to_predicate` not found for this
[00:10:03]     |
[00:10:03]     = help: items from traits can only be used if the trait is in scope
[00:10:03]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:10:03]             candidate #1: `use ty::ToPredicate;`
[00:10:03] 
[00:10:03] error[E0599]: no method named `subst` found for type `&'tcx ty::TyS<'tcx>` in the current scope
[00:10:03]    --> librustc/traits/object_safety.rs:346:46
[00:10:03]     |
[00:10:03] 346 |         let target_receiver_ty = receiver_ty.subst(
[00:10:03]     |
[00:10:03]     |
[00:10:03]     = help: items from traits can only be used if the trait is in scope
[00:10:03]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:10:03]             candidate #1: `use ty::subst::Subst;`
