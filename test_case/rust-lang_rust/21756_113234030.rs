
test.rs:5:1: 9:2 error: the object type `MyTrait + 'a` automatically implements the trait `MyTrait` [E0371]
test.rs:5 impl<'a> MyTrait for MyTrait + 'a {
test.rs:6     fn do_something(&self) {
test.rs:7         println!("MyTrait");
test.rs:8     }
test.rs:9 }
test.rs:5:1: 9:2 help: run `rustc --explain E0371` to see a detailed explanation
error: aborting due to previous error
