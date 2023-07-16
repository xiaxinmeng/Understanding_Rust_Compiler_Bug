plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error: expected expression, found `=`
   --> compiler/rustc_typeck/src/check/demand.rs:546:37
    |
546 | ...                   if  = sm.span_to_snippet(left_expr.span).is_ok() {
    |                           ^ expected expression
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: aborting due to previous error

error: could not compile `rustc_typeck`
