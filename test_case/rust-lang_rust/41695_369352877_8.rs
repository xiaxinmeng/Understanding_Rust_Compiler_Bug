
error[E0594]: cannot assign to captured outer variable in an `FnMut` closure
  --> $DIR/closure-immutable-outer-variable.rs:19:26
   |
LL |     let y = true;
   |         - help: consider making `y` mutable: `mut y`
LL |     foo(Box::new(move || y = false) as Box<_>);
   |                          ^^^^^^^^^ cannot assign to captured outer variable in an `FnMut` closure
