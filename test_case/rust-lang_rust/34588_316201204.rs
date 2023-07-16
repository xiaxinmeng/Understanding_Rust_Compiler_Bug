bash
vagrant@rust-lang error_index $ cargo run
   Compiling error_index v0.1.0 (file:///home/vagrant/repos/error_index)
register_long_diagnostics! { E001 : r##"hello"## , E002 : r##"world"## , }
register_diagnostic! { E001 , r##"hello"## }
println! { "code is {}" , stringify ! ( E001 ) }
error: macros that expand to items must either be surrounded with braces or followed by a semicolon
 --> <println macros>:3:16
  |
3 | ) => ( print ! ( concat ! ( $ fmt , "\n" ) , $ ( $ arg ) * ) ) ;
  |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

print! { concat ! ( "code is {}" , "\n" ) , stringify ! ( E001 ) }
error: expected one of `!` or `::`, found `(`
 --> <print macros>:2:25
  |
2 | $ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;
  |                        -^ unexpected token
  |                        |
  |                        expected one of `!` or `::` here

error: Could not compile `error_index`.
