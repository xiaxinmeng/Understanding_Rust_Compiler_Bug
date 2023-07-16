
error[E0530]: match bindings cannot shadow tuple structs
  --> src/main.rs:20:27
   |
11 | use X::CrateNum;
   |     ----------- a tuple struct `CrateNum` is imported here
...
20 |         Enum::CrateNumVal(CrateNum) => {
   |                           ^^^^^^^^ cannot be named the same as a tuple struct
