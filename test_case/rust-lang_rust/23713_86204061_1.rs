
foo.rs:7:16: 7:17 error: use of moved value: `x`
foo.rs:7     assert_eq!(x.is_some(), true);
                        ^
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
foo.rs:7:5: 7:35 note: expansion site
foo.rs:6:16: 6:17 note: `x` moved here because it has type `core::option::Option<Foo>`, which is non-copyable
foo.rs:6     assert_eq!(x.unwrap(), Foo);
                        ^
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
foo.rs:6:5: 6:33 note: expansion site
