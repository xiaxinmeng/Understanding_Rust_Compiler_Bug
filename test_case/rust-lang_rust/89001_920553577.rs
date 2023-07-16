plain
    Checking clippy_lints v0.1.57 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/escape.rs:105:13
    |
105 |             trait_self_ty,
    |             ^^^^^^^^^^^^^ expected `&TyS<'_>`, found struct `Binder`
    |
    = note: expected enum `std::option::Option<&TyS<'_>>`
               found enum `std::option::Option<Binder<'_, &TyS<'_>>>`
error[E0308]: mismatched types
    --> src/tools/clippy/clippy_lints/src/methods/mod.rs:2068:21
     |
2068 |                     self_ty,
2068 |                     self_ty,
     |                     ^^^^^^^ expected `&TyS<'_>`, found struct `Binder`
     |
     = note: expected reference `&TyS<'_>`
                   found struct `Binder<'_, &TyS<'_>>`
error[E0308]: mismatched types
    --> src/tools/clippy/clippy_lints/src/methods/mod.rs:2082:45
     |
     |
2082 |             if !contains_ty(cx.tcx, ret_ty, self_ty);
     |                                             ^^^^^^^ expected `&TyS<'_>`, found struct `Binder`
     |
     = note: expected reference `&TyS<'_>`
                   found struct `Binder<'_, &TyS<'_>>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints` due to 3 previous errors
Build completed unsuccessfully in 0:04:21
