plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: no rules expected the token `*`
    --> src/librustdoc/html/render/print_item.rs:1300:58
     |
1300 |         .filter(|f| matches!(clean::StructFieldItem(..), *f.kind))
     |                                                          ^ no rules expected this token in macro call
error: unexpected end of macro invocation
    --> src/librustdoc/html/render/print_item.rs:1361:74
     |
     |
1361 |                 .filter(|f| matches!(clean::StructFieldItem(..) = *f.kind))
     |                                                                          ^ missing tokens in macro arguments
error: aborting due to 2 previous errors

error: could not compile `rustdoc`

