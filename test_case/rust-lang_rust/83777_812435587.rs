plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/html/render/mod.rs:865:10
    |
865 |     tcx: TyCtxt<'_>,
    |
help: consider importing one of these items
    |
35  | use crate::TyCtxt;
35  | use crate::TyCtxt;
    |
35  | use rustc_middle::ty::TyCtxt;
    |

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/html/render/mod.rs:1272:61
     |
1272 | fn spotlight_decl(decl: &clean::FnDecl, cache: &Cache, tcx: TyCtxt<'_>) -> String {
     |
help: consider importing one of these items
     |
35   | use crate::TyCtxt;
35   | use crate::TyCtxt;
     |
35   | use rustc_middle::ty::TyCtxt;
     |

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/html/render/mod.rs:2052:10
     |
2052 |     tcx: TyCtxt<'_>,
     |
help: consider importing one of these items
     |
35   | use crate::TyCtxt;
35   | use crate::TyCtxt;
     |
35   | use rustc_middle::ty::TyCtxt;
     |

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/html/render/mod.rs:2064:10
     |
2064 |     tcx: TyCtxt<'_>,
     |
help: consider importing one of these items
     |
35   | use crate::TyCtxt;
