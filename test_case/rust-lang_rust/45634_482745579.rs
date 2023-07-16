
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
 --> file.rs:7:18
  |
7 |         Box::new(move || println!("{}", self.name))
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 6:5...
 --> file.rs:6:5
  |
6 | /     fn foo(&self) -> Box<Fn()> {
7 | |         Box::new(move || println!("{}", self.name))
8 | |     }
  | |_____^
  = note: ...so that the types are compatible:
          expected &Foo
             found &Foo
  = note: but, the lifetime must be valid for the static lifetime...
  = note: ...so that the expression is assignable:
          expected std::boxed::Box<(dyn std::ops::Fn() + 'static)>
             found std::boxed::Box<dyn std::ops::Fn()>
