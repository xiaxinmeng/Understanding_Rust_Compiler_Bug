plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:102:17
    |
38  | ) -> Result<(), NotConstEvaluatable> {
    |      ------------------------------- expected `Result<(), rustc_middle::thir::abstract_const::NotConstEvaluatable>` because of return type
...
102 |                 NotConstEvaluatable::MentionsInfer
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `rustc_middle::thir::abstract_const::NotConstEvaluatable`
    |
    = note: expected enum `Result<(), rustc_middle::thir::abstract_const::NotConstEvaluatable>`
               found enum `rustc_middle::thir::abstract_const::NotConstEvaluatable`
help: try wrapping the expression in `Err`
    |
102 |                 Err(NotConstEvaluatable::MentionsInfer)
    |                 ++++                                  +
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to previous error
