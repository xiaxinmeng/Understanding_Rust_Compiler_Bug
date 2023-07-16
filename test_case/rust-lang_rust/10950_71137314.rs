
hello.rs:11:29: 11:34 error: type `Wrap<B>` does not implement any method in scope named `bar`
hello.rs:11                     (*self).bar().foo(x);
                                        ^~~~~
hello.rs:11:34: 11:34 help: methods from traits can only be called if the trait is implemented and in scope; the following trait defines a method `bar`, perhaps you need to implement it:
hello.rs:11:34: 11:34 help: candidate #1: `Bar`
hello.rs:11:41: 11:41 help: methods from traits can only be called if the trait is implemented and in scope; the following trait defines a method `foo`, perhaps you need to implement it:
hello.rs:11:41: 11:41 help: candidate #1: `Foo`

