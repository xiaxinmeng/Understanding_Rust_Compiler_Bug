
foo.rs:10:61: 10:64 error: failed to find an implementation of trait std::clone::Clone for [int, .. 2]
foo.rs:10         fn foo<C:Clone>(x: C) {} let x: [int, .. 2] = [1,2]; foo(x)
                                                                       ^~~
note: in expansion of fmt!
foo.rs:9:10: 11:7 note: expansion site
application terminated with error code 101
