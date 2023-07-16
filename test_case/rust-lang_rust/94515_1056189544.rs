
error[E0507]: cannot move out of `abc`, a captured variable in an `FnMut` closure
 --> f33.rs:3:34
  |
2 |     let mut abc = Abc { bar: Vec::new() };
  |         ------- captured outer variable
3 |     items.iter().for_each(|item| abc.update(*item));
  |                           -------^^^--------------
  |                           |      |
  |                           |      move occurs because `abc` has type `Abc`, which does not implement the `Copy` trait
  |                           captured by this `FnMut` closure

warning: variable does not need to be mutable
 --> f33.rs:2:9
  |
2 |     let mut abc = Abc { bar: Vec::new() };
  |         ----^^^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

error[E0382]: use of moved value: `abc`
 --> f33.rs:5:5
  |
2 |     let mut abc = Abc { bar: Vec::new() };
  |         ------- move occurs because `abc` has type `Abc`, which does not implement the `Copy` trait
3 |     items.iter().for_each(|item| abc.update(*item));
  |                           ------ --- variable moved due to use in closure
  |                           |
  |                           value moved into closure here
4 |     //~^ ERROR cannot move out of `abc`, a captured variable in an `FnMut` closure
5 |     abc //~ ERROR use of moved value: `abc`
  |     ^^^ value used here after move
