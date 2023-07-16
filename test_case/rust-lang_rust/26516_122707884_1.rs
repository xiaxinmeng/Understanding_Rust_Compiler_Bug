
> rustc --version
rustc 1.3.0-nightly (e4e93196e 2015-07-14)
> rustc foo.rs
foo.rs:8:38: 8:46 error: no method named `unwrap` found for type `core::result::Result<&str, Foo>` in the current scope
foo.rs:8     assert_eq!("just for test", res1.unwrap()); //error
                                              ^~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
foo.rs:8:5: 8:48 note: expansion site
foo.rs:8:38: 8:46 note: the method `unwrap` exists but the following trait bounds were not satisfied: `Foo : core::fmt::Debug`
