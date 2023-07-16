plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0308]: mismatched types
   --> compiler/rustc_session/src/errors.rs:341:69
    |
341 |                 && looks_like_base_prefixed_number(symbol.as_str(), fix_base_capitalisation(suf)) {
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

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_session` due to previous error
warning: build failed, waiting for other jobs to finish...
