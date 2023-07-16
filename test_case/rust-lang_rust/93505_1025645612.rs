plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `expect_ty` found for reference `&TyS<'_>` in the current scope
   --> src/librustdoc/clean/utils.rs:112:52
    |
112 |             ty::Tuple(tys) => tys.iter().map(|t| t.expect_ty().clean(cx)).collect(),
    |                                                    ^^^^^^^^^ method not found in `&TyS<'_>`

error[E0599]: no method named `expect_ty` found for reference `&TyS<'_>` in the current scope
    --> src/librustdoc/clean/mod.rs:1537:54
     |
1537 |             ty::Tuple(t) => Tuple(t.iter().map(|t| t.expect_ty().clean(cx)).collect()),
     |                                                      ^^^^^^^^^ method not found in `&TyS<'_>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:02:40
