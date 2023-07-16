
<std macros>:1:33: 1:58 error: invalid reference to argument `0` (no arguments given)
<std macros>:1 ( $ fmt : expr ) => ( print ! ( concat ! ( $ fmt , "\n" ) ) ) ; (
                                               ^~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:1:33: 1:58 note: in this expansion of concat!
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:1:23: 1:60 note: in this expansion of print! (defined in <std macros>)
test.rs:3:25: 3:39 note: in this expansion of println! (defined in <std macros>)
test.rs:2:26: 2:37 note: in this expansion of m! (defined in test.rs)
test.rs:6:1: 6:35 note: in this expansion of m! (defined in test.rs)
error: aborting due to previous error
