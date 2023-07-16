
bad2.rs:7:12: 7:18 warning: unused variable: `reader`, #[warn(unused_variables)] on by default
bad2.rs:7 pub fn bar(reader: &mut Reader) {
                     ^~~~~~
bad2.rs:10:1: 16:2 warning: function `new_Foo` should have a snake case name such as `new_foo`, #[warn(non_snake_case)] on by default
bad2.rs:10 pub fn new_Foo (reader: Box<Reader +'static>) -> Foo {
bad2.rs:11     let mut foo = Foo {
bad2.rs:12         reader: reader
bad2.rs:13     };
bad2.rs:14     bar(&mut foo.reader);
bad2.rs:15     foo
           ...
