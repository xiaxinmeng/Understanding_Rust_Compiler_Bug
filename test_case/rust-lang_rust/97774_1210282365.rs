plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0522]: definition of an unknown language item: `owned_box_new`
    |
    |
210 |     #[lang = "owned_box_new"]
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ definition of unknown language item `owned_box_new`
For more information about this error, try `rustc --explain E0522`.
error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:17
