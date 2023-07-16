
error[E0308]: arguments to this function are incorrect
  --> $DIR/wrong-call-return-type-due-to-generic-arg.rs:27:5
   |
LL |     function(0u32, 8u8)
   |     ^^^^^^^^ ----  --- expected `bool`, found `u8`
   |              |
   |              expected `()`, found `u32`
   |
help: the return type of this call is `u32` due to the type of the argument passed
  --> $DIR/wrong-call-return-type-due-to-generic-arg.rs:27:5
   |
LL |     function(0u32, 8u8)
   |     ^^^^^^^^^----^^^^^^
   |                  |
   |                  this argument determines the return type of `function`
note: function defined here
  --> $DIR/wrong-call-return-type-due-to-generic-arg.rs:1:4
   |
L  | fn function<T, N>(x: T, y: bool) -> T {
   |    ^^^^^^^^       ----  -------
