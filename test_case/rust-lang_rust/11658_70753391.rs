
hello.rs:4:10: 4:46 error: type parameter `T` shadows another type parameter of the same name
hello.rs:4          fn new<T>(x: T) -> Foo<T> { Foo(x) }
                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

