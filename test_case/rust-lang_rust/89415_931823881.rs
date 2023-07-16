plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0277]: `rustc_span::Span` doesn't implement `std::fmt::Display`
   --> compiler/rustc_mir_transform/src/inline.rs:150:61
    |
150 |               format!("Inline {:?} into {}", callsite.callee, caller_body.span)
    |               |                                               |
    |               |                                               `rustc_span::Span` cannot be formatted with the default formatter
    |               in this macro invocation (#1)
    |
---
    | |_- in this expansion of `format!` (#1)
    |
   ::: /checkout/library/core/src/macros/mod.rs:835:5
    |
835 | /     macro_rules! format_args {
836 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
837 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `$crate::__export::format_args!` (#2)
    |
    = help: the trait `std::fmt::Display` is not implemented for `rustc_span::Span`
    = help: the trait `std::fmt::Display` is not implemented for `rustc_span::Span`
    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
