
[00:05:51]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:11] error[E0599]: no method named `parse_pat_tuple_elements` found for type `&mut parse::parser::Parser<'a>` in the current scope
[00:06:11]     --> libsyntax/parse/parser.rs:3783:40
[00:06:11]      |
[00:06:11] 3783 |             if let Err(mut err) = self.parse_pat_tuple_elements(false) {
[00:06:11]      |                                        ^^^^^^^^^^^^^^^^^^^^^^^^
[00:06:11]      |
[00:06:11]      = help: did you mean `parse_pat_vec_elements`?
[00:06:11] 
[00:06:13] error: aborting due to previous error
[00:06:13] 
[00:06:13] error: Could not compile `syntax`.
