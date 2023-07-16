
warning: method `Bar` should have a snake case name such as `bar`
 --> ...\test.rs:4:5
  |
4 | /     fn Bar(x: i32) -> Foo {
5 | |         println!("I'm uncallable");
6 | |         Foo::Bar(x)
7 | |     }
  | |_____^
  |
  = note: #[warn(non_snake_case)] on by default
