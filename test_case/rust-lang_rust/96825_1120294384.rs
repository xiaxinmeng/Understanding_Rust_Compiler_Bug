plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `visit_all_item_likes` found for struct `rustc_middle::hir::map::Map` in the current scope
    |
    |
306 |         tcx.hir().visit_all_item_likes(&mut finder);
    |                   ^^^^^^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `par_visit_all_item_likes`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:37
