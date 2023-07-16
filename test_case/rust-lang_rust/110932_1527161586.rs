plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
error[E0599]: no method named `source` found for enum `libloading::Error` in the current scope
    --> compiler/rustc_metadata/src/creader.rs:1099:49
     |
1099 |     let message = if let Some(src) = last_error.source() {
     |
    ::: /checkout/library/core/src/error.rs:84:8
     |
     |
84   |     fn source(&self) -> Option<&(dyn Error + 'static)> {
     |        ------ the method is available for `libloading::Error` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
3    | use std::error::Error;
