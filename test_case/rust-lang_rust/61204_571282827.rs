
error[E0425]: cannot find value `B` in module `my_lib`
  --> file17.rs:21:22
   |
4  |             pub struct A;
   |             ------------- similarly named unit struct `A` defined here
...
21 |     let _b = my_lib::B;
   |                      ^
   |
help: a unit struct with a similar name exists
   |
21 |     let _b = my_lib::A;
   |                      ^
help: possible candidate is found in another module, you can import it into scope
   |
1  | use my_lib::inner_mod::inner_level::B;
   |

warning: unused import: `self::inner_level::B`
 --> file17.rs:9:13
  |
9 |         use self::inner_level::B;
  |             ^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
