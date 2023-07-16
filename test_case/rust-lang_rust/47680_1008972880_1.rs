
error[E0499]: cannot borrow `*opt` as mutable more than once at a time
 --> main.rs:4:17
  |
1 |   fn get_default<'a, T: Default>(opt: &'a mut Option<T>) -> &'a mut T {
  |                  -- lifetime `'a` defined here
2 |       match opt.as_mut() {
  |       -     ------------ first mutable borrow occurs here
  |  _____|
  | |
3 | |         Some(value) => value,
4 | |         None => opt.insert(T::default()),
  | |                 ^^^^^^^^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
5 | |     }
  | |_____- returning this value requires that `*opt` is borrowed for `'a`
