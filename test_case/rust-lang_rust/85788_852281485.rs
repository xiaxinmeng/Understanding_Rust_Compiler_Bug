plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/config.rs:631:13
    |
631 |         let (lint_opts, describe_lints, lint_cap) = get_cmd_lint_options(matches, error_format);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^   ------------------------------------------- this expression has type `(Vec<(std::string::String, rustc_lint_defs::Level)>, bool, std::option::Option<rustc_lint_defs::Level>, Vec<std::string::String>)`
    |             |
    |             expected a tuple with 4 elements, found one with 3 elements
    |
    = note: expected tuple `(Vec<(std::string::String, rustc_lint_defs::Level)>, bool, std::option::Option<rustc_lint_defs::Level>, Vec<std::string::String>)`
               found tuple `(_, _, _)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc`
