
error[E0597]: lifetime error
  --> src/main.rs:9:14
   |
9  |     foo(|a| &x)
   |          -  ^^ `x` would have to be valid for the lifetime `'1`
   |          |
   |          let's call the type of this `&'1 u32`
   |  }
   |  - but `x` goes out of scope here
