
error: attempted to take value of method `inputs` on type `&rustc::ty::FnSig<'_>`

   --> /checkout/src/librustc_typeck/check/demand.rs:118:47

    |

118 |                         fty.sig.skip_binder().inputs.len() == 1

    |                                               ^^^^^^

    |

    = help: maybe a `()` to call it is missing? If not, try an anonymous function
