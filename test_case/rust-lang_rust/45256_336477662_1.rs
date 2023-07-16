
<std macros>:3:11: 3:36 error: invalid reference to argument `0` (no arguments given)
<std macros>:3 print ! ( concat ! ( $ fmt , "\n" ) , $ ( $ arg ) * ) ) ;
                         ^~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:3:11: 3:36 note: in this expansion of concat!
<std macros>:2:27: 2:58 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
test.rs:2:5: 2:27 note: in this expansion of println! (defined in <std macros>)
test.rs:2:24: 2:25 error: named argument never used
test.rs:2     println!("{}", x = 1);
                                 ^
error: aborting due to 2 previous errors
