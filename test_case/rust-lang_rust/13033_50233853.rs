
$ rustc r13033.rs 
r13033.rs:8:5: 8:38 error: method `bar` has an incompatible type for trait: values differ in mutability [E0053]
r13033.rs:8     fn bar(&mut self, other: &Foo) {}
                ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
