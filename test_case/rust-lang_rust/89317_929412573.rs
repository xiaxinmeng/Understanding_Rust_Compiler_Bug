plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0599]: no method named `unwrap_or` found for struct `rustc_span::Span` in the current scope
   --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:253:30
    |
253 |             .span_label(span.unwrap_or(self.root_span()), msg)

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
