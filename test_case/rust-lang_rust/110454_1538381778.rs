plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0599]: no method named `bound_explicit_item_bounds` found for struct `TyCtxt<'tcx>` in the current scope
   --> compiler/rustc_ty_utils/src/opaque_types.rs:110:30
    |
108 |                           for (pred, _span) in self
109 | |                             .tcx
109 | |                             .tcx
110 | |                             .bound_explicit_item_bounds(alias_ty.def_id)
    | |                             -^^^^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `explicit_item_bounds`
    | 

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_ty_utils` (lib) due to previous error
