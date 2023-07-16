plain
[00:06:41]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:44]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:06:44]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:48]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:57] error: description for error code E0541 contains a line longer than 80 characters.
[00:06:57] if you're inserting a long URL use the footnote style to bypass this check.
[00:06:57]    --> libsyntax/diagnostics/macros.rs:13:37
[00:06:57]     |
[00:06:57] 13  |       ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
[00:06:57]     | 
[00:06:57]     | 
[00:06:57]    ::: libsyntax/diagnostic_list.rs:16:1
[00:06:57]     |
[00:06:57] 16  | / register_long_diagnostics! {
[00:06:57] 17  | |
[00:06:57] 18  | | E0178: r##"
[00:06:57] 19  | | In types, the `+` type operator has low precedence, so it is often necessary
[00:06:57] 345 | |
[00:06:57] 346 | | }
[00:06:57]     | |_- in this macro invocation
[00:06:57] 
