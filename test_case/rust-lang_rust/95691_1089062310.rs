plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/html/render/context.rs:445:55
    |
445 |                     layout.external_html.in_header += s.to_string();
    |                                                       |
    |                                                       expected `&str`, found struct `std::string::String`
    |                                                       expected `&str`, found struct `std::string::String`
    |                                                       help: consider borrowing here: `&s.to_string()`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:02:33
