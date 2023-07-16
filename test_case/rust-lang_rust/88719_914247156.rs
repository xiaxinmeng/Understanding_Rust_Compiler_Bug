plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:494:32
    |
494 |             std::rc::Rc::clone(parent_code)
    |
    = note: expected reference `&Rc<_>`
    = note: expected reference `&Rc<_>`
               found reference `&Arc<rustc_middle::traits::ObligationCauseCode<'_>>`
error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:683:32
    |
    |
683 |             std::rc::Rc::clone(parent_code)
    |
    = note: expected reference `&Rc<_>`
    = note: expected reference `&Rc<_>`
               found reference `&Arc<rustc_middle::traits::ObligationCauseCode<'_>>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
