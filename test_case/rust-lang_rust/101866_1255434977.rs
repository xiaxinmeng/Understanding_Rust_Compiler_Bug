
error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
  --> main.rs:15:5
   |
2  |     fn func();
   |     ---------- `TraitA::func` defined here
...
15 |     TraitA::<i32>::func();
   |     ^^^^^^^^^^^^^^^^^^^ cannot call associated function of trait
   |
help: use the fully-qualified path to the only available implementation
   |
15 |     <::StructA as TraitA::<i32>>::func();
   |     +++++++++++++              +

error: aborting due to previous error
