plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/clean/types.rs:453:52
    |
453 |                 let other_attrs = cx.tcx.get_attrs(inlined_id).clean(cx);
    |                                                    ^^^^^^^^^^ expected struct `rustc_span::def_id::DefId`, found struct `LocalDefId`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:06
