
note: trace_macro
  --> ../../src/test/run-pass/log_syntax-trace_macros-macro-locations.rs:27:5
   |
27 |     println!("");
   |     ^^^^^^^^^^^^^ expands to `println! { "" }`
   |     ^^^^^^^^^^^^^ expands to `print! { concat ! ( "" , "\n" ) }`
