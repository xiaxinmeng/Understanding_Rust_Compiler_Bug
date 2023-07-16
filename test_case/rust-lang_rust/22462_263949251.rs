
error[E0530]: let bindings cannot shadow statics
  --> <anon>:3:13
   |
3  |         let value = $e;
   |             ^^^^^ cannot be named the same as a static
...
9  |     static value: i32 = 4; // comment out this line and things work
   |     ---------------------- a static `value` is defined here
10 |     let x = foo!(3);
   |             ------- in this macro invocation
