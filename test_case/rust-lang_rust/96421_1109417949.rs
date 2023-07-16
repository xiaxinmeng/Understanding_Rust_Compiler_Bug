plain
    Checking structopt v0.3.25
error[E0308]: mismatched types
    --> src/tools/rustfmt/src/expr.rs:1328:58
     |
1328 |                 rustc_ast::ast::MacDelimiter::from_token(mac.args.delim()),
     |                                                          ^^^^^^^^^^^^^^^^ expected enum `DelimToken`, found enum `std::option::Option`
     |
     = note: expected enum `DelimToken`
                found enum `std::option::Option<DelimToken>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustfmt-nightly` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustfmt-nightly` due to previous error
