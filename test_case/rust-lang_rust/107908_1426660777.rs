plain
    Checking cargo_metadata v0.15.3
    Checking rustfix v0.6.1
    Checking clippy_lints v0.1.69 (/checkout/src/tools/clippy/clippy_lints)
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
error[E0004]: non-exhaustive patterns: `rustc_type_ir::sty::TyKind::DynStar(_, _)` not covered
     |
     |
1387 |         break match *ty.kind() {
     |                     ^^^^^^^^^^ pattern `rustc_type_ir::sty::TyKind::DynStar(_, _)` not covered
     |
note: `rustc_type_ir::sty::TyKind<TyCtxt<'_>>` defined here
    --> /checkout/compiler/rustc_type_ir/src/sty.rs:132:5
     |
50   | pub enum TyKind<I: Interner> {
...
...
132  |     DynStar(I::ListBinderExistentialPredicate, I::Region),
     |     ^^^^^^^ not covered
     = note: the matched value is of type `rustc_type_ir::sty::TyKind<TyCtxt<'_>>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
1428 ~             }
1428 ~             }
1429 ~             rustc_type_ir::sty::TyKind::DynStar(_, _) => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:02:50
