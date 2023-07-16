
test.rs:11:10: 11:15 error: type `T` does not implement any method in scope named `foo`
test.rs:11     data.foo()
                    ^~~~~
test.rs:11:15: 11:15 note: found defined static methods, maybe a `self` is missing?
test.rs:2:5: 2:13 note: candidate #1 is `Foo::foo`
test.rs:2     fn foo();
              ^~~~~~~~
error: aborting due to previous error
