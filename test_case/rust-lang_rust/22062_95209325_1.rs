 shell
$ rustc main.rs
main.rs:50:9: 50:21 warning: unused variable: `foocontainer`, #[warn(unused_variables)] on by default
main.rs:50     let foocontainer: FooContainer<F> = FooContainer { foo: Foo { member: ptr::null_mut() } };
                   ^~~~~~~~~~~~
$ echo $?
0
