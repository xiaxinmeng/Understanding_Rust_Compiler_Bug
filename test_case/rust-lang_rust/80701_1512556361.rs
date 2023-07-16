
error[[E0521]](https://doc.rust-lang.org/stable/error_codes/E0521.html): borrowed data escapes outside of associated function
  --> src/main.rs:33:8
   |
31 | impl <'a>TestHolder<'a>{
   |       -- lifetime `'a` defined here
32 |     pub fn test_foo(&self){
   |                     ----- `self` is a reference that is only valid in the associated function body
33 |        self.holder.as_ref().unwrap().borrow_mut().set(20); 
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |        |
   |        `self` escapes the associated function body here
   |        argument requires that `'a` must outlive `'static`
   |
   = note: requirement occurs because of a mutable reference to `dyn FooSetter`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
   