plain
[00:07:37]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:07:42]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:07:43]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:46]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:54] error: used diagnostic code E0452 not registered
[00:07:54]    --> libsyntax/diagnostics/macros.rs:99:9
[00:07:54]     |
[00:07:54] 99  |           __diagnostic_used!($code);
[00:07:54]     | 
[00:07:54]    ::: libsyntax/attr/builtin.rs:465:31
[00:07:54]     |
[00:07:54]     |
[00:07:54] 465 |                   let mut err = struct_span_err!(
[00:07:54]     |  _______________________________-
[00:07:54] 466 | |                     sess.span_diagnostic,
[00:07:54] 467 | |                     lit.span,
[00:07:54] 468 | |                     E0452,
[00:07:54] 469 | |                     "literal in `cfg` predicate value must be a string",
[00:07:54]     | |_________________- in this macro invocation
[00:07:54] 
