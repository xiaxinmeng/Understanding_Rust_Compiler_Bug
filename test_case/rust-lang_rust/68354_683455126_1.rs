
error[E0007]: cannot bind by-move with sub-bindings
  --> $DIR/E0007.rs:6:9
   |
LL |         op_string @ Some(s) => {},
   |         ^^^^^^^^^^^^^^^^^^^ binds an already bound by-move value by moving it
