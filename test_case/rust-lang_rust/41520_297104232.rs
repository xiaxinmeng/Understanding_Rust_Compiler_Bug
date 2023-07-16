
note: trace_macro
  --> ../../src/test/run-pass/log_syntax-trace_macros-macro-locations.rs:27:5
   |
27 |     println!("");
   |     ^^^^^^^^^^^^^ tracing macro expansion here
   |
   | note: expands to `println! { "" }`
   | note: expands to `print! { concat ! ( "" , "\n" ) }`
