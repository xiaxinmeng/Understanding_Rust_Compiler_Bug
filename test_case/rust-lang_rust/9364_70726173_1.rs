
stmt_list! { while x < 100 { let y = x + 1 ; x = y ; } }
hello.rs:4:64: 4:72 error: unexpected token: `an interpolated tt`
hello.rs:4     ( while $cond:expr { $($body:tt)+ } ) => ( while $cond { $($body:tt)+ } );
                                                                          ^~~~~~~~

