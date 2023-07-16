plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/check.rs:1393:62
     |
1393 |             let display_discr = format_discriminant_overflow(discr, tcx, v);
     |                                                              ^^^^^ expected struct `TyCtxt`, found struct `Discr`
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/check.rs:1393:69
     |
     |
1393 |             let display_discr = format_discriminant_overflow(discr, tcx, v);
     |                                                                     ^^^ expected `&rustc_hir::Variant<'_>`, found struct `TyCtxt`
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/check.rs:1393:74
     |
     |
1393 |             let display_discr = format_discriminant_overflow(discr, tcx, v);
     |                                                                          ^ expected struct `Discr`, found `&rustc_hir::Variant<'_>`
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/check.rs:1394:64
     |
     |
1394 |             let display_discr_i = format_discriminant_overflow(disr_vals[i], tcx, variant_i);
     |                                                                ^^^^^^^^^^^^ expected struct `TyCtxt`, found struct `Discr`
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/check.rs:1394:78
     |
     |
1394 |             let display_discr_i = format_discriminant_overflow(disr_vals[i], tcx, variant_i);
     |                                                                              ^^^ expected `&rustc_hir::Variant<'_>`, found struct `TyCtxt`
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/check.rs:1394:83
     |
     |
1394 |             let display_discr_i = format_discriminant_overflow(disr_vals[i], tcx, variant_i);
     |                                                                                   ^^^^^^^^^ expected struct `Discr`, found `&rustc_hir::Variant<'_>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_typeck` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_typeck` due to 6 previous errors
