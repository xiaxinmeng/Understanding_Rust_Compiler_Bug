plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unused `Result` that must be used
   --> src/librustdoc/html/render/print_item.rs:685:13
    |
685 |             w.write_str(" · ");
    |
    |
    = note: `-D unused-must-use` implied by `-D warnings`
    = note: this `Result` may be an `Err` variant, which should be handled
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:01
