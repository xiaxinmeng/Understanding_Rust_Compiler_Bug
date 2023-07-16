plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: `if` and `else` have incompatible types
   --> src/librustdoc/html/format.rs:368:21
359 | /                 if indent == 0 {
359 | /                 if indent == 0 {
360 | |                     format!(" <br><span class=\"where\">where{where_preds}</span>");
    | |                                                                                    - help: consider removing this semicolon
361 | |                 } else {
362 | |                     let mut clause = String::with_capacity(6 * indent - 2);
368 | |                     clause
    | |                     ^^^^^^ expected `()`, found struct `std::string::String`
369 | |                 }
369 | |                 }
    | |_________________- `if` and `else` have incompatible types
   ::: /checkout/library/alloc/src/macros.rs:111:23
    |
111 |       ($($arg:tt)*) => {{
    |  _______________________-
