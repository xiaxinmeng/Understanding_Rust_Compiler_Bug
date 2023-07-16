
test.rs:15:10: 15:15 error: no method named `foo` found for type `i32` in the current scope
test.rs:15     0i32.foo();
                    ^~~~~
test.rs:15:10: 15:15 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
test.rs:15:10: 15:15 help: candidate #1: use `inner::Foo`
error: aborting due to previous error
