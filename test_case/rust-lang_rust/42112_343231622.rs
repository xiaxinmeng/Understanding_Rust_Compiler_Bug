
error[E0507]: cannot move out of captured outer variable in an `Fn` closure
  --> foo.rs:10:9
   |
8  | fn just_do_it(my_arg_1: Rc<RefCell<MyType>>, ...
   |               -------- captured outer variable 
   |
9  |     exec_closure(move || {
10 |         my_arg_1;
   |         ^^^^^^^^ cannot move out of captured outer variable in an `Fn` clos
ure
