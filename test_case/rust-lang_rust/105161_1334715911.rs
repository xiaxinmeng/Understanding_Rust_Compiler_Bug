plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0425]: cannot find value `buf` in this scope
    |
    |
341 |                 && looks_like_base_prefixed_number(symbol.as_str(), fix_base_capitalisation(buf)) {
    |                                                                                             ^^^ help: a local variable with a similar name exists: `suf`
error[E0308]: mismatched types
   --> compiler/rustc_session/src/errors.rs:341:69
    |
    |
341 |                 && looks_like_base_prefixed_number(symbol.as_str(), fix_base_capitalisation(buf)) {
    |                    -------------------------------                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found enum `Option`
    |                    arguments to this function are incorrect
    |
    = note: expected reference `&str`
                    found enum `Option<std::string::String>`
                    found enum `Option<std::string::String>`
note: function defined here
   --> compiler/rustc_session/src/errors.rs:295:8
    |
295 |     fn looks_like_base_prefixed_number(prefix: &str, suffix: &str) -> bool {

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_session` due to 2 previous errors
