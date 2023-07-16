
foo.rs:7:9: 7:19 error: type `Foo` does not implement any method in scope named `function`
foo.rs:7     Foo.function();
                 ^~~~~~~~~~
foo.rs:7:19: 7:19 note: found defined static methods, maybe a `self` is missing?
foo.rs:3:5: 3:21 note: candidate #1 is `Foo::function`
foo.rs:3     fn function() {}
             ^~~~~~~~~~~~~~~~
error: aborting due to previous error
