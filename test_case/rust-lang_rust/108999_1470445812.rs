rust
error: lifetime may not live long enough
  --> src/main.rs:18:9
   |
16 |     fn mut_rc<'s>(&'s mut self) -> Option<&'s mut (dyn Module + 's)> {
   |               -- lifetime `'s` defined here
17 |         // this doesn't work
18 |         Rc::get_mut(&mut self.module_rc)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'s` must outlive `'static`
   |
   = note: requirement occurs because of a mutable reference to `dyn Module`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
