
note: trace_macro
  --> $DIR/trace-macro.rs:14:5
   |
14 |     println!("Hello, World!");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: expanding this macro call to:
           `println! { "Hello, World!" }`
           `print ! ( concat ! ( "Hello, World!" , "/n" ) )`
           `$crate :: io :: _print ( format_args ! ( concat ! ( "Hello, World!" , "/n" ) )
           )`
