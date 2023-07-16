plain
   Compiling synstructure v0.12.4
   Compiling tracing-attributes v0.1.13
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.55.0
error[E0405]: cannot find trait `MultiSpan` in crate `proc_macro`
   --> compiler/rustc_macros/src/session_diagnostic.rs:109:36
    |
109 | fn span_err(span: impl proc_macro::MultiSpan, msg: &str) -> proc_macro::Diagnostic {
    |                                    ^^^^^^^^^ not found in `proc_macro`
help: consider importing this trait
    |
2   | use proc_macro::bridge::server::MultiSpan;
    |
    |

error[E0405]: cannot find trait `MultiSpan` in crate `proc_macro`
   --> compiler/rustc_macros/src/session_diagnostic.rs:131:51
    |
131 |     #[cfg(not(bootstrap))] span: impl proc_macro::MultiSpan,
    |                                                   ^^^^^^^^^ not found in `proc_macro`
help: consider importing this trait
    |
2   | use proc_macro::bridge::server::MultiSpan;
    |
