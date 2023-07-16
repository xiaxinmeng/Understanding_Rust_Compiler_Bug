
<anon>:1:32: 1:33 error: the trait bound `T: std::fmt::Display` is not satisfied [E0277]
<anon>:1 fn g<T>(x: T) { println!("{}", x); }
                                        ^
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
<anon>:1:17: 1:35 note: in this expansion of println! (defined in <std macros>)
<anon>:1:32: 1:33 help: see the detailed explanation for E0277
<anon>:1:32: 1:33 help: consider adding a `where T: std::fmt::Display` bound
<anon>:1:32: 1:33 note: required by `std::fmt::Display::fmt`
error: aborting due to previous error
