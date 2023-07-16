plain
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0596]: cannot borrow `flush_last_line` as mutable, as it is not declared as mutable
    |
    |
160 |                 let flush_last_line = |last_frame, times| {
    |                     --------------- help: consider changing this to be mutable: `mut flush_last_line`
161 |                     if let Some((line, span)) = last_frame {
162 |                         err.span_label(span, &line);
    |                         --- calling `flush_last_line` requires mutable binding due to mutable borrow of `err`
...
184 |                         flush_last_line(last_frame, times);
    |                         ^^^^^^^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `flush_last_line` as mutable, as it is not declared as mutable
    |
    |
160 |                 let flush_last_line = |last_frame, times| {
    |                     --------------- help: consider changing this to be mutable: `mut flush_last_line`
161 |                     if let Some((line, span)) = last_frame {
162 |                         err.span_label(span, &line);
    |                         --- calling `flush_last_line` requires mutable binding due to mutable borrow of `err`
...
189 |                 flush_last_line(last_frame, times);
    |                 ^^^^^^^^^^^^^^^ cannot borrow as mutable
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
For more information about this error, try `rustc --explain E0596`.
error: could not compile `rustc_const_eval` due to 2 previous errors
