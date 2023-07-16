
<anon>:1:17: 1:18 error: binary operation `+` cannot be applied to type `T` [E0369]
<anon>:1 fn f<T>(x: T) { x + 4; }
                         ^
<anon>:1:17: 1:18 help: see the detailed explanation for E0369
<anon>:2:32: 2:33 error: the trait `core::fmt::Display` is not implemented for the type `T` [E0277]
<anon>:2 fn g<T>(x: T) { println!("{}", x); }
                                        ^
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
<anon>:2:17: 2:35 note: in this expansion of println! (defined in <std macros>)
<anon>:2:32: 2:33 help: see the detailed explanation for E0277
<anon>:2:32: 2:33 note: `T` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
<anon>:2:32: 2:33 note: required by `core::fmt::Display::fmt`
<anon>:3:48: 3:50 error: the trait `h::X` is not implemented for the type `T` [E0277]
<anon>:3 fn h<T>(x: T) { trait X {} fn hh<T:X>(x: T) {} hh(x); }
                                                        ^~
<anon>:3:48: 3:50 help: see the detailed explanation for E0277
<anon>:3:48: 3:50 note: required by `h::hh`
