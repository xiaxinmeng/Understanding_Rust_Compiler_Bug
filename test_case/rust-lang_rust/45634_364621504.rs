
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
 --> src/main.rs:7:9
  |
7 |         Box::new(move || println!("{}", self.name))
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 6:5...
 --> src/main.rs:6:5
  |
6 | /     fn foo(&self) -> Box<Fn()> {
7 | |         Box::new(move || println!("{}", self.name))
8 | |     }
  | |_____^
note: ...so that the type `[closure@src/main.rs:7:18: 7:51 self:&Foo]` will meet its required lifetime bounds
 --> src/main.rs:7:9
  |
7 |         Box::new(move || println!("{}", self.name))
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: but, the lifetime must be valid for the static lifetime...
  = note: ...so that the expression is assignable:
          expected std::boxed::Box<std::ops::Fn() + 'static>
             found std::boxed::Box<std::ops::Fn()>
