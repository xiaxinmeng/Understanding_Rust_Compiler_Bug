
error[E0597]: lifetime error
  --> src/main.rs:9:14
   |
9  |     foo(|a| &x)
   |          -  ^^ return value has to have the lifetime `'1`
   |          |
   |          let's call the type of this `&'1 u32`
   |
