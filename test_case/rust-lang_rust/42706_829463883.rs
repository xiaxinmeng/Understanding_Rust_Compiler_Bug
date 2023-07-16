
error[E0623]: lifetime mismatch
  --> src/lib.rs:10:20
   |
8  |     fn foo<'a>(x: &i32, y: &'a i32) -> &'a i32 {
   |                   ----                 -------
   |                   |
   |                   this parameter and the return type are declared with different lifetimes...
9  |         // this body `y` would be OK
10 |         if x > y { x } else { y }
   |                    ^ ...but data from `x` is returned here
