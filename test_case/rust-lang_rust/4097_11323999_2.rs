
$ rustc foo.rs 
warning: no debug symbols in executable (-arch x86_64)

$ rustc bar.rs -L .
bar.rs:5:17: 5:25 error: unresolved name
bar.rs:5     assert 42 == Foo::foo();
                          ^~~~~~~~
bar.rs:5:17: 5:25 error: use of undeclared module `Foo`
bar.rs:5     assert 42 == Foo::foo();
                          ^~~~~~~~
bar.rs:5:17: 5:25 error: unresolved name: Foo::foo
bar.rs:5     assert 42 == Foo::foo();
                          ^~~~~~~~
error: aborting due to 3 previous errors
