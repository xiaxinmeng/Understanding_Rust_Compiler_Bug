
hello.rs:3:1: 3:22 warning: struct is never used: `Foo`, #[warn(dead_code)] on by default
hello.rs:3 struct Foo { x: Int }
           ^~~~~~~~~~~~~~~~~~~~~
hello.rs:3:14: 3:20 warning: struct field is never used: `x`, #[warn(dead_code)] on by default
hello.rs:3 struct Foo { x: Int }
                        ^~~~~~
