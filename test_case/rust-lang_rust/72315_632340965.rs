
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> test.rs:45:14
   |
45 |         let (marg1, marg2) = marg;
   |              ^^^^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the method body at 44:5...
  --> test.rs:44:5
   |
44 | /     fn foo(&self, marg: &mut Self::MutArg, _varg: Self::ValArg) {
45 | |         let (marg1, marg2) = marg;
46 | |         let subfoo = (self.factory)();
47 | |         subfoo.foo(
...  |
62 | |         );
63 | |     }
   | |_____^
note: ...so that reference does not outlive borrowed content
  --> test.rs:45:14
   |
45 |         let (marg1, marg2) = marg;
   |              ^^^^^
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the expression is assignable
  --> test.rs:61:13
   |
61 |             Box::new(move |subfun| subfun(marg1)),
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected  `std::boxed::Box<(dyn std::ops::FnOnce(std::boxed::Box<(dyn for<'r> std::ops::FnOnce(&'r mut MutArg1) + 'static)>) + 'static)>`
              found  `std::boxed::Box<dyn std::ops::FnOnce(std::boxed::Box<(dyn for<'r> std::ops::FnOnce(&'r mut MutArg1) + 'static)>)>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0495`.
