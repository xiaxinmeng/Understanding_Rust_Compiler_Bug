 shell
$ rustc main.rs
main.rs:1:1: 1:12 warning: module `Foo` should have a snake case name such as `foo`, #[warn(non_snake_case)] on by default
main.rs:1 mod Foo { }
          ^~~~~~~~~~~
main.rs:2:1: 2:12 warning: struct is never used: `Foo`, #[warn(dead_code)] on by default
main.rs:2 struct Foo;
          ^~~~~~~~~~~
