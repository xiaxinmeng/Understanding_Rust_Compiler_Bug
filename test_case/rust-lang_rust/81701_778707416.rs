
error[EXXXX]: cannot call associated function on trait without specifying the corresponding `impl` type
  --> src/main.rs:12:5
   |
2  |     fn my_fn();
   |     ----------- `MyTrait::my_fn` defined here
...
12 |     MyTrait::my_fn();
   |     ^^^^^^^^^^^^^^ cannot call associated function of trait
   |
help: use a fully-qualified path to a specific available implementation
   |
12 |     <MyStruct as MyTrait>::my_fn();
   |     ^^^^^^^^^^^^^       ^
