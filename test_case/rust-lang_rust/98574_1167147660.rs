plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error[E0050]: method `visit_local` has 2 parameters but the declaration in trait `rustc_hir::intravisit::Visitor::visit_local` has 3
    --> compiler/rustc_privacy/src/lib.rs:1277:20
     |
1277 |     fn visit_local(&mut self, local: &'tcx hir::Local<'tcx>) {
     |
     |
     = note: `visit_local` from trait: `fn(&mut Self, &'v rustc_hir::Local<'v>, Option<&'v rustc_hir::Block<'v>>)`
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_privacy/src/lib.rs:1285:9
     |
     |
1285 |         intravisit::walk_local(self, local);
     |         ^^^^^^^^^^^^^^^^^^^^^^------------- an argument of type `Option<&rustc_hir::Block<'_>>` is missing
note: function defined here
    --> /checkout/compiler/rustc_hir/src/intravisit.rs:493:8
     |
     |
493  | pub fn walk_local<'v, V: Visitor<'v>>(
help: provide the argument
     |
     |
1285 |         intravisit::walk_local(self, local, {Option<&rustc_hir::Block<'_>>});

Some errors have detailed explanations: E0050, E0061.
For more information about an error, try `rustc --explain E0050`.
error: could not compile `rustc_privacy` due to 2 previous errors
