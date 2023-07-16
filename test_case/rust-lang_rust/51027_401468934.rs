
error[E0597]: `x` is owned by the closure, so you cannot return a reference to it
  --> $DIR/issue-11925.rs:18:35
   |
LL |         let f = to_fn_once(move|| &x); //~ ERROR does not live long enough
   |                                   ^
   |                                   |
   |                                   |
| cannot return reference to data owned by the closure
