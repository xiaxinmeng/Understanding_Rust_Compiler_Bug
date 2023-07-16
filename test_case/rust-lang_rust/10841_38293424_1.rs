
% rustc /tmp/demo.rs
/tmp/demo.rs:4:41: 4:45 error: internal compiler error: cannot relate bound region: ReScope(43) <= ReEarlyBound(42, 0, a)
/tmp/demo.rs:4 fn check<'a, F: Foo<'a>>(cont: &'a F) { cont.foo(); }
                                                       ^~~~
note: the compiler hit an unexpected failure path. this is a bug.
