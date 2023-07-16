
foo.rs:6:10: 6:15 error: type `T` does not implement any method in scope named `foo`
foo.rs:6     data.foo()
                      ^~~~~
foo.rs:6:15: 6:15 note: found defined static methods, maybe a `self` is missing?
foo.rs:2:5: 2:13 note: candidate #1 is defined in the trait `Foo`
foo.rs:2     fn foo();
