plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0616]: field `place` of struct `interpret::place::PlaceTy` is private
    |
    |
369 |     trace!("{:?}", ecx.dump_place(place.place));


error[E0616]: field `mplace` of struct `MPlaceTy` is private
    |
    |
216 |             let ref_place = pointee_place.mplace.to_ref(&tcx);

    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
For more information about this error, try `rustc --explain E0616`.
