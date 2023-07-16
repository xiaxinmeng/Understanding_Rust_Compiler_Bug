bash
error: macro expansion ignores token `register_diagnostic` and any following
  --> t.rs:25:15
   |
25 |             $(register_diagnostic! { $code, $description })*
   |               ^^^^^^^^^^^^^^^^^^^
   |
note: caused by the macro expansion here; the usage of `register_long_diagnostics!` is likely invalid in expression context
  --> ./diagnostics.rs:1:1
   |
1  | / register_long_diagnostics! {
2  | |     E001: r##"hello"##,
3  | |     E002: r##"world"##,
4  | | }
   | |_^

note: trace_macro
  --> t.rs:29:5
   |
29 |     include! { "./diagnostics.rs" }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: expanding `register_long_diagnostics! { E001 : r##"hello"## , E002 : r##"world"## , }`
   = note: to `register_diagnostic ! { E001 , r##"hello"## } register_diagnostic ! {
           E002 , r##"world"## }`
   = note: expanding `register_diagnostic! { E001 , r##"hello"## }`
   = note: to `println ! ( "code is {}" , stringify ! ( E001 ) ) ;`
   = note: expanding `println! { "code is {}" , stringify ! ( E001 ) }`
   = note: to `print ! ( concat ! ( "code is {}" , "\n" ) , stringify ! ( E001 ) )`
   = note: expanding `print! { concat ! ( "code is {}" , "\n" ) , stringify ! ( E001 ) }`
   = note: to `$crate :: io :: _print (
           format_args ! ( concat ! ( "code is {}" , "\n" ) , stringify ! ( E001 ) ) )`

error: aborting due to previous error(s)
