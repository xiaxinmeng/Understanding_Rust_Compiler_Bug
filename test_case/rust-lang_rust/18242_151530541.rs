
hello.rs:1:22: 1:23 error: expected `,` or `>` after lifetime name, found `+`
hello.rs:1 fn f() -> Box<'static+std::fmt::Show> { unreachable!() }
                                ^
hello.rs:1:22: 1:23 error: expected type, found `+`
hello.rs:1 fn f() -> Box<'static+std::fmt::Show> { unreachable!() }
                                ^
hello.rs:1:15: 1:23 note: did you mean a single argument type &'a Type, or did you mean the comma-separated arguments 'a, Type?
hello.rs:1 fn f() -> Box<'static+std::fmt::Show> { unreachable!() }
                         ^~~~~~~~
