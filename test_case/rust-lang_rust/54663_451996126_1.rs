
Compiling playground v0.0.1 (/playground)
error[E0506]: cannot assign to `self.0` because it is borrowed
  --> src/main.rs:72:9
   |
35 |     fn get_or_set(&mut self, value: WrappedI32) -> &mut WrappedI32 {
   |                   - let's call the lifetime of this reference `'1`
...
59 |             let a = self.0.as_mut();
   |                     ------ borrow of `self.0` occurs here
60 |             if 1 == 2 {
61 |                 return a.unwrap();
   |                        ---------- returning this value requires that `self.0` is borrowed for `'1`
...
72 |         self.0 = Opt::Some(value); //compile fails is here: error[E0506]: cannot assign to `self.0` because it is borrowed
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `self.0` occurs here

error: aborting due to previous error
