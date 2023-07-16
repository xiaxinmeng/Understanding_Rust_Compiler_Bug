rust
    macro_rules! register_diagnostics {
        ($($code:tt),*) => {{
            $(register_diagnostic! { $code })*
        }};
        ($($code:tt),*,) => {{
            $(register_diagnostic! { $code })*
        }};
    }
